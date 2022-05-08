use anyhow::{self, Result};
use weh_lib::{background, check_feh, cli::interface, download};
#[tokio::main]
async fn main() -> Result<()> {
    check_feh::check()?;
    let a = interface::input()?;
    println!("{:?}", a);
    download::download_img(
        String::from("https://georgik.rocks/wp-content/uploads/sianim.gif"),
        String::from("test.gif"),
    )
    .await?;

    background::set_background(&a);
    Ok(())
}
