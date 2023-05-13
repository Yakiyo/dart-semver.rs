use crate::channel::Channel;

pub struct Version {
    pub channel: Channel,
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub prerelease: Option<usize>,
    pub prerelease_patch: Option<usize>,
}

impl Version {
    fn _parse<P: AsRef<str>>(_s: P) {}
}

impl std::str::FromStr for Version {
    type Err = &'static str;

    fn from_str(_s: &str) -> Result<Self, Self::Err> {
        Err("Invalid format")
    }
}
