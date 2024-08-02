use std::fs;
use native_dialog::FileDialog;

pub fn create_new_project() -> Result<(), Box<dyn std::error::Error>> {
    let path = FileDialog::new()
        .set_location("~")
        .add_filter("Project Directory", &[""])
        .show_open_single_dir()?;

    if let Some(path) = path {
        let project_name = "New Project";
        let project_path = path.join(project_name);

        fs::create_dir_all(&project_path)?;
        println!("Created new project at: {:?}", project_path);
    }

    Ok(())
}