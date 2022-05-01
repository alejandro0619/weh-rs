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
