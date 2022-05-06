use anyhow::{
    self, 
    Result
};
use weh_lib::{
    background, 
    check_feh, 
    download,
    helper::dirs
};

#[tokio::main]
async fn main() -> Result<()> {
    check_feh::check()?;
    let path = dirs::picture_folder()?;
    download::download_img(
        String::from("https://georgik.rocks/wp-content/uploads/sianim.gif"),
        String::from(format!("{}/test.gif", path)),
    )
    .await?;
    
    background::set_background();
    Ok(())
}
