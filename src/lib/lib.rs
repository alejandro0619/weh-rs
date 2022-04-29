/// This module contains a single function in charge to verify whether ``feh`` exists in the system or not.
pub mod check_feh {

    use crate::{check_os::{self, check_os}, distro::DistrosAvailable};
    use anyhow::Result;
    use std::process::Command;

    /// It only works for arch based distros because we run pacman to check if the package is installed.

    pub fn check() -> Result<bool> {
        /*if check_os::check_os().is_ok() {
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
                    eprintln!("{:?}", e);
                    Err(anyhow::anyhow!("{}", e))
                }
            }
        } else {
            std::process::exit(1)
        }*/
        match check_os::check_os() {
            Ok(()) => match check_os::check_distro() {
                Ok(distro_name) => {
                    match distro_name {
                        DistrosAvailable::Arch => {
                            Ok(true)
                        }, 
                        DistrosAvailable::Debian => {
                            Ok(true)
                        },
                        DistrosAvailable::Other(name) => {
                            Err(anyhow::anyhow!(" The distro {} is not supported", name))
                        }
                    }
                },
                Err(e) => Err(e),
            },
            Err(error) => Err(anyhow::anyhow!("Description message: {:?}", error)),
        }
    }
}
mod distro {
    #[derive(Debug)]
    pub enum DistrosAvailable {
        Arch,
        Debian,
        Other(String), // This will throw an error later: Distro is not supported
    }
    impl DistrosAvailable {
        pub fn new(name: &str) -> Self {
            match name {
                "debian" => DistrosAvailable::Debian,
                "arch" => DistrosAvailable::Arch,
                _ => DistrosAvailable::Other(String::from(name)),
            }
        }
    }
}
pub mod check_os {
    use crate::distro::DistrosAvailable;
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
            Err(anyhow::anyhow!("{} is not currently supported!", os)) // Err! if its winbug or macos
        } else {
            Ok(())
            // cool
        }
    }

    pub fn check_distro() -> anyhow::Result<DistrosAvailable> {
        let id_like_distro = sys_info::linux_os_release()
            .with_context(|| "Failed to gather information")?
            .id_like
            .expect("Failed to read from /etc/os-release");

        Ok(DistrosAvailable::new(&id_like_distro))
    }
}
