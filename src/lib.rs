#![doc = include_str!("../README.md")]

mod channel;
mod parser;

pub use channel::*;

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
