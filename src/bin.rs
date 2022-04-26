use anyhow::Result;
use weh_lib::check_feh;
fn main() -> Result<()> {
    if check_feh::check()? {
        println!("Feh is installed")
    } else {
        eprintln!("Feh is not installed");
        std::process::exit(1);
    };
    Ok(())
}
