use directories::UserDirs;
use anyhow::{Result, Context};

pub fn picture_folder() -> Result<String>{
  // Initialize a new instance.
  let user_dir = UserDirs::new().with_context(|| "Could not find HOME directory... aborting")?;
  let pic_dir = user_dir.picture_dir().with_context(|| "Could not find Pictures direcoty.")?;
  Ok(pic_dir.to_string_lossy().to_string())
}