use crate::check_sys_info::{check_distro, check_os, check_wm, distro};
use anyhow::Result;
use std::process::{Command, Stdio};

/// Runs ``both check_os`` and ``check_distro`` to verify the system info, and then checks if feh is installed: Only works in debian-based and arch-based distros
pub fn check() -> Result<()> {
    match check_os::check() {
        Ok(_) => match check_distro::check() {
            Ok(distro_name) => match distro_name {
                distro::DistrosAvailable::Arch => {
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
                        // At this point checks if the wm is supported
                        match check_wm::check() {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e),
                        }
                    }
                }
                distro::DistrosAvailable::Debian => {
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
                        match check_wm::check() {
                            Ok(_) => Ok(()),
                            Err(e) => Err(e),
                        }
                    }
                }
                distro::DistrosAvailable::Other(name) => {
                    Err(anyhow::anyhow!("The distro {} is not supported", name))
                }
            },
            Err(e) => Err(e),
        },
        Err(error) => Err(anyhow::anyhow!("Description message: {:?}", error)),
    }
}
