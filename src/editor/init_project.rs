use std::path::{Path, PathBuf};
use std::process::Command;
use std::fs;

#[cfg(target_family = "unix")]
use std::os::unix::fs::PermissionsExt;

#[cfg(target_family = "windows")]
use std::os::windows::prelude::*;

pub fn init_bevy_project(project_path: &Path) -> bool {
    let init_output = Command::new("cargo")
        .args(&["init", "--bin", project_path.to_str().unwrap()])
        .output();

    match init_output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Cargo init failed with error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return false;
            }
        }
        Err(e) => {
            eprintln!("Failed to execute cargo init: {}", e);
            return false;
        }
    }

    let add_output = Command::new("cargo")
        .current_dir(project_path)
        .args(&["add", "bevy"])
        .output();

    match add_output {
        Ok(output) => {
            if !output.status.success() {
                eprintln!(
                    "Cargo add bevy failed with error: {}",
                    String::from_utf8_lossy(&output.stderr)
                );
                return false;
            }
        }
        Err(e) => {
            eprintln!("Failed to execute cargo add bevy: {}", e);
            return false;
        }
    }

    #[cfg(target_family = "unix")]
    {
        if let Ok(metadata) = fs::metadata(project_path) {
            let mut permissions = metadata.permissions();
            permissions.set_mode(0o755);
            if let Err(e) = fs::set_permissions(project_path, permissions) {
                eprintln!("Failed to set permissions: {}", e);
            }
        } else {
            eprintln!("Failed to get metadata for path: {:?}", project_path);
        }
    }

    #[cfg(target_family = "windows")]
    {
        Command::new("icacls")
            .args(&[project_path.to_str().unwrap(), "/grant", "Everyone:(OI)(CI)F"])
            .output()
            .expect("Failed to set permissions");
    }

    true
}