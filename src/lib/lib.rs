/// This module contains a single function in charge to verify whether ``feh`` exists in the system or not.
pub mod check_feh {

    use crate::check_os;
    use anyhow::Result;
    use std::process::Command;

    /// It only works for arch based distros because we run pacman to check if the package is installed.

    pub fn check() -> Result<bool> {
        if check_os::check_os().is_ok() {
            match check_os::check_distro() {
                Ok(name) => {
                    if name == "debian" {
                        let success = Command::new("dpkg").args(["-s", "feh"]).status()?;
                        if success.code() != Some(0) {
                            Err(anyhow::anyhow!("Feh is not installed!")) // If the code is not equal to zero, return false
                        } else {
                            Ok(true)
                        }
                    } else {
                        let success = Command::new("pacman").args(["-Q", "feh"]).status()?;
                        if success.code() != Some(0) {
                            Err(anyhow::anyhow!("Feh is not installed!")) // If the code is not equal to zero, return false
                        } else {
                            Ok(true)
                        }
                    }
                }
                Err(e) => {
                    Err(anyhow::anyhow!("Something went wrong! {}", e))
                }
            }
        } else {
            std::process::exit(1)
        }
    }
}
pub mod check_os {
    use anyhow::Context;
    use sys_info;

    ///
    ///
    /// # Examples
    ///
    /// ```
    /// use weh_lib::check_os::check_os;
    ///
    /// assert_eq!(check_os(), true); // It is supported
    /// ```
    ///
    /// # Panics
    ///
    /// Panics if you can't retrieve info from the os at etc/os-release.
    pub fn check_os() -> anyhow::Result<()> {
        let os = sys_info::os_type().context("Failed to get the system information")?;

        if &os == "Darwin" || &os == "Windows" {
            let a = anyhow::anyhow!(
                "The current system is not supported! {}",
                os);
                eprintln!("{:?}", a);
            Err(anyhow::anyhow!(
                "The current system is not supported! {}",
                os
            )) // Err! if its winbug or macos
        } else {
            Ok(()) // cool
        }
    }

    pub fn check_distro() -> anyhow::Result<String> {
        let id_like_distro = sys_info::linux_os_release()
            .context("Failed to gather information")?
            .id_like;
        Ok(id_like_distro.expect("Can't found sys info"))
    }
}
