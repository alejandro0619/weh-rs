use anyhow::{self, Result};
use weh_lib::{background, check_feh, cli::interface, download};
#[tokio::main]
async fn main() -> Result<()> {
    check_feh::check()?;
    let path = interface::input();
    println!("{}", &path.0);

    if path.1 {
        download::download_img(String::from(&path.0), String::from("test.png")).await?;
    }
    else {
        background::set_background(&path.0);
    }
    Ok(())
}
