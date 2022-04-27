use anyhow::Result;
use weh_lib::check_feh;
fn main() -> Result<()> {
    check_feh::check()?;
    Ok(())
}
