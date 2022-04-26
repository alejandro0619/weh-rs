/// This module contains a single function in charge to verify whether ``feh`` exists in the system or not.
pub mod check_feh {
    use anyhow::Result;
    use std::process::Command;
    /// It only works for arch based distros because we run pacman to check if the package is installed.
    pub fn check() -> Result<bool> {
        let success = Command::new("pacman").args(["-Q", "fehkk"]).status()?;
        if success.code() != Some(0) {
            Ok(false) // If the code is not equal to zero, return false
        } else {
            Ok(true)
        }
    }
}
