/// This module contains a single function in charge to verify whether ``feh`` exists in the system or not.
pub mod check_feh {

    use crate::{
        check_os::{check_distro, check_os},
        distro::DistrosAvailable,
    };
    use anyhow::Result;
    use std::process::{Command, Stdio};

    /// Runs ``both check_os`` and ``check_distro`` to verify the system info, and then checks if feh is installed: Only works in debian-based and arch-based distros
    pub fn check() -> Result<()> {
        match check_os() {
            Ok(()) => match check_distro() {
                Ok(distro_name) => match distro_name {
                    DistrosAvailable::Arch => {
                        let success = Command::new("pacman")
                            .args(["-Q", "feh"])
                            .stdout(Stdio::null()) //Will not generate any default input, but the one the program will throw
                            .output()?
                            .status
                            .code();
                        if success != Some(0) {
                            Err(anyhow::anyhow!(
                                "Feh is not installed, exit with code: {:?}",
                                success.unwrap_or(1)
                            ))
                        } else {
                            Ok(())
                        }
                    }
                    DistrosAvailable::Debian => {
                        let success = Command::new("dpkg")
                            .args(["-s", "feh"])
                            .stdout(Stdio::null()) //Will not generate any default input, but the one the program will throw
                            .output()?
                            .status
                            .code();

                        if success != Some(0) {
                            Err(anyhow::anyhow!(
                                "Feh is not installed, exit with code: {:?}",
                                success.unwrap_or(1)
                            ))
                        } else {
                            Ok(())
                        }
                    }
                    DistrosAvailable::Other(name) => {
                        Err(anyhow::anyhow!("The distro {} is not supported", name))
                    }
                },
                Err(e) => Err(e),
            },
            Err(error) => Err(anyhow::anyhow!("Description message: {:?}", error)),
        }
    }
}
/// Distros availables
mod distro {
    #[derive(Debug)]
    /// List of distros that are supported
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
    ///  Checks that the current OS is linux, if not, sends an error
    pub fn check_os() -> anyhow::Result<()> {
        let os = sys_info::os_type().context("Failed to get the system information")?;
        if &os == "Darwin" || &os == "Windows" {
            Err(anyhow::anyhow!("{} is not currently supported!", os)) // Err! if its winbug or macos
        } else {
            Ok(())
            // cool
        }
    }
    // Checks the current distro, it must be debian-based or arch-based systems
    pub fn check_distro() -> anyhow::Result<DistrosAvailable> {
        let id_like_distro = sys_info::linux_os_release()
            .with_context(|| "Failed to gather information")?
            .id_like
            .with_context(|| "Can't read ID_LIKE from the file : /etc/os-release");

        match id_like_distro {
            Ok(id_like) => Ok(DistrosAvailable::new(&id_like)),
            Err(e) => Err(e),
        }
    }
}
