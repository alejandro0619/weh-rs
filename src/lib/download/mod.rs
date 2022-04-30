use anyhow::{self, Context, Result};
use std::{fs::File, io::Cursor};

pub async fn download_img(url: String, file_name: String) -> Result<()> {
    let response = reqwest::get(url)
        .await
        .with_context(|| "An error ocurred when fetching the image.")?;

    let mut file = File::create(&file_name)
        .with_context(|| format!("Couldn't create the file at {}", file_name))?;

    let mut content = Cursor::new(response.bytes().await?);

    std::io::copy(&mut content, &mut file)
    .with_context(|| "Failed to copy the file, try again.")?;
    
    Ok(())
}