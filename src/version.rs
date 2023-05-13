use crate::channel::Channel;

/// A struct representing a version.
/// 
/// General format of dart-sdk's version - x.y.z-a.b.channel
/// 
/// Reference: https://github.com/dart-lang/sdk/blob/main/tools/VERSION
#[derive(PartialEq, Debug)]
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
