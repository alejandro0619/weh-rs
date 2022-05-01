use anyhow::Context;
use sys_info;
///  Checks that the current OS is linux, if not, sends an error
pub fn check() -> anyhow::Result<()> {
    let os = sys_info::os_type().context("Failed to get the system information")?;
    if &os == "Darwin" || &os == "Windows" {
        Err(anyhow::anyhow!("{} is not currently supported!", os)) // Err! if its winbug or macos
    } else {
        Ok(()) // It's cool
    }
}
