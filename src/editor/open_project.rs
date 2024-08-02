use rfd::FileDialog;
use std::path::PathBuf;

/// Opens a dialog to select an existing project directory.
///
/// # Returns
///
/// An `Option<PathBuf>` where `Some(PathBuf)` contains the path to the selected
/// directory or `None` if no directory was selected.
pub fn open_existing_project() -> Option<PathBuf> {
    // Open the file dialog to select an existing project directory
    FileDialog::new()
        .set_title("Select Existing Project Directory")
        .pick_folder()
}