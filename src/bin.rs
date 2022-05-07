use anyhow::{
    self, 
    Result
};
use weh_lib::{
    check_feh, 
    download,
};

#[tokio::main]
async fn main() -> Result<()> {
    check_feh::check()?;
    
    download::download_img(
        String::from("https://georgik.rocks/wp-content/uploads/sianim.gif"),
        String::from("test.gif")
    )
    .await?;
    
    //background::set_background();
    Ok(())
}
