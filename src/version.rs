use std::str::FromStr;

use crate::channel::Channel;
use crate::parser::{Rule, VersionParser};
use pest::error as perror;

/// A struct representing a version.
///
/// General format of dart-sdk's version - x.y.z-a.b.channel
///
/// Reference: https://github.com/dart-lang/sdk/blob/main/tools/VERSION
#[derive(PartialEq, Debug, Default, Copy)]
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
        matches!(self.channel, Channel::Stable)
    }

    /// Parse a version struct from a string
    pub fn parse<P: AsRef<str>>(s: P) -> Result<Version, Box<perror::Error<Rule>>> {
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
    type Err = Box<perror::Error<Rule>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Version::parse(s)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::convert::From<&str> for Version {
    fn from(value: &str) -> Self {
        Version::from_str(value).unwrap()
    }
}

impl Clone for Version {
    fn clone(&self) -> Self {
        Version::parse(self.to_str()).unwrap()
    }
}

#[cfg(test)]
mod test {
    use super::{Version, Channel};

    #[test]
    fn version_test() {
        assert_eq!(
            Version::parse("4.3.4").unwrap(),
            Version {
                major: 4,
                minor: 3,
                patch: 4,
                prerelease: None,
                prerelease_patch: None,
                channel: Channel::Stable
            }
        )
    }
}
