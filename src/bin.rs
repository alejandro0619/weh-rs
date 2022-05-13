use anyhow::{self, Result};
use weh_lib::{  
    background,
    check_feh, 
    cli::interface,
    download
};
#[tokio::main]

async fn main() -> Result<()> {
    check_feh::check()?;
    let path = interface::input();

    if path.1 {
        download::download_img(String::from(&path.0), String::from(&path.2.unwrap())).await?;
        // ofc I will replace that unwrap()
    }
    else {
        background::set_background(&path.0);
    }
    Ok(())
}

