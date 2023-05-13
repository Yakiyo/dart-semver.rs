use crate::channel::Channel;
use crate::parser::{Rule, VersionParser};
use pest::error as perror;

/// A struct representing a version.
///
/// General format of dart-sdk's version - x.y.z-a.b.channel
///
/// Reference: https://github.com/dart-lang/sdk/blob/main/tools/VERSION
#[derive(PartialEq, Debug, Default)]
pub struct Version {
    pub channel: Channel,
    pub major: usize,
    pub minor: usize,
    pub patch: usize,
    pub prerelease: Option<usize>,
    pub prerelease_patch: Option<usize>,
}

impl Version {
    /// Wether the current version is a non-stable build or not
    pub fn is_stable(&self) -> bool {
        match self.channel {
            Channel::Stable => true,
            _ => false,
        }
    }

    /// Parse a version struct from a string
    pub fn parse<P: AsRef<str>>(s: P) -> Result<Version, perror::Error<Rule>> {
        VersionParser::version(s)
    }

    /// Convert version to string
    pub fn to_str(&self) -> String {
        let mut s = format!("{}.{}.{}", self.major, self.minor, self.patch);
        if !self.is_stable() {
            s += format!(
                "-{}.{}.{}",
                self.prerelease.unwrap(),
                self.prerelease_patch.unwrap(),
                self.channel
            )
            .as_str();
        }
        s
    }
}

impl std::str::FromStr for Version {
    type Err = perror::Error<Rule>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Version::parse(s)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}
