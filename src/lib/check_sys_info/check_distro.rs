use super::distro::DistrosAvailable;
use anyhow::Context;
use sys_info;

// Checks the current distro, it must be debian-based or arch-based systems
pub fn check() -> anyhow::Result<DistrosAvailable> {
    let id_like_distro = sys_info::linux_os_release()
        .with_context(|| "Failed to gather information")?
        .id_like
        .with_context(|| "Can't read ID_LIKE property from the file : /etc/os-release");

    match id_like_distro {
        Ok(id_like) => Ok(DistrosAvailable::new(&id_like)),
        Err(e) => Err(e),
    }
}
