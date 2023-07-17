use crate::channel::Channel;
use crate::parser::Rule;
use crate::parser::VersionParser;

/// An enum of possible version formats
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum Version {
    /// Major, Minor, patch - v2.3.4
    FullStable(usize, usize, usize),
    /// Major, minor - v3.1
    MajorMinor(usize, usize),
    /// Major v3
    MajorOnly(usize),
    /// Major, Minor, Patch, Prerelease, Prerelease-patch, Channel - v3.1.2-4.5.beta
    NonStable(usize, usize, usize, usize, usize, Channel),
}

impl Version {
    /// Parse version from a string
    pub fn parse<S: AsRef<str>>(s: S) -> Result<Version, Box<pest::error::Error<Rule>>> {
        VersionParser::version(s.as_ref())
    }
    /// If version is stable or not
    pub fn is_stable(&self) -> bool {
        !matches!(self, Self::NonStable(..))
    }

    /// Returns the channel of the version string
    pub fn channel(&self) -> Channel {
        match self {
            Self::NonStable(.., c) => *c,
            _ => Channel::Stable,
        }
    }

    /// String representation of the version
    pub fn as_string(&self) -> String {
        match self {
            Self::MajorOnly(major) => format!("v{major}"),
            Self::MajorMinor(major, minor) => format!("v{major}.{minor}"),
            Self::FullStable(major, minor, patch) => format!("v{major}.{minor}.{patch}"),
            Self::NonStable(ma, mi, pa, pr, prp, c) => {
                format!("v{ma}.{mi}.{pa}-{pr}.{prp}.{c}")
            }
        }
    }
}

impl std::str::FromStr for Version {
    type Err = Box<pest::error::Error<Rule>>;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Version::parse(s)
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

impl std::default::Default for Version {
    fn default() -> Self {
        Version::MajorOnly(0)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn version_test() {
        assert_eq!(
            Version::parse("v3.4.5").unwrap(),
            Version::FullStable(3, 4, 5)
        );
    }

    #[test]
    fn stable_test() {
        assert!(Version::parse("v3").unwrap().is_stable());
        assert!(!Version::parse("v3.4.5-1.2.beta").unwrap().is_stable());
    }

    #[test]
    fn format_str_test() {
        assert_eq!(
            Version::parse("v3.4.5").unwrap().as_string(),
            String::from("v3.4.5")
        );
        assert_eq!(
            Version::parse("v3.4.5-1.2.dev").unwrap().as_string(),
            String::from("v3.4.5-1.2.dev")
        );
        assert_ne!(
            Version::parse("v3.4.5-1.2.dev").unwrap().as_string(),
            String::from("v3.4.5")
        );
    }
}
