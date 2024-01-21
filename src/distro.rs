use os_release::OsRelease;

pub enum Distro {
    Generic,
    ArchLinux,
    Debian,
}

impl Distro {
    pub fn new() -> Self {
        let release = OsRelease::new().unwrap();

        Self::from_name(release.id)
    }

    fn from_name(name: String) -> Self {
        match name.as_str() {
            "arch" => Distro::ArchLinux,
            "debian" => Distro::Debian,
            _ => Distro::Generic,
        }
    }
}
