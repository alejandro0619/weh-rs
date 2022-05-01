use anyhow::{Context, Result};
use std::process::Command;

pub fn check() -> Result<()> {
    let wm_de: Vec<u8> = Command::new("bash")
        .arg("check_wm::check().sh")
        .output()
        .with_context(|| "Failed obtaining the WM/DE identify")?
        .stdout;

    //Validate:
    if String::from_utf8_lossy(&wm_de).trim() != "bspwm" {
        Err(anyhow::anyhow!("The current DE/WM is not supported"))
    } else {
        Ok(())
    }
}
