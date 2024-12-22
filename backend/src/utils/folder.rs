use std::fs;

pub const STORAGE_FOLDER: &str = "data";

pub fn create_necessary_folders() -> loco_rs::Result<()> {
    fs::create_dir_all(STORAGE_FOLDER)?;

    Ok(())
}
