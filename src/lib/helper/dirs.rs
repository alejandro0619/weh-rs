use anyhow::{Context, Result};
use directories::UserDirs;
use std::fs;
use std::path::PathBuf;

/// Retrieve Pictures folder inside the User folder.
pub fn picture_folder() -> Result<PathBuf> {
    // Initialize a new instance.
    let user_dir = UserDirs::new().with_context(|| "Could not find HOME directory... aborting")?;
    Ok((user_dir
        .picture_dir()
        .with_context(|| "Could not find Pictures directory.")?)
    .to_path_buf())
}

/// Creates and retrieves the path of a new folder created at the Pictures folder.
pub fn create_custom_folder(pic_dir: PathBuf) -> Result<PathBuf> {
    let wallp_dir = pic_dir.join("wallpapers");

    if wallp_dir.exists() {
        Ok(wallp_dir)
    } else {
        fs::create_dir(&wallp_dir).with_context(|| "Couldn't create the wallpaper folder.")?;
        Ok(wallp_dir)
    }
}
