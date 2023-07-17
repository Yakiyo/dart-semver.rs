use crate::channel::Channel;

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
    /// If version is stable or not
    pub fn is_stable(&self) -> bool {
        if matches!(self, Self::NonStable(..)) {
            false
        } else {
            true
        }
        // match self {
        //     Self::NonStable(..) => false,
        //     _ => true,
        // }
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

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}
