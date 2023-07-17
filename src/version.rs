use crate::channel::Channel;

pub enum Version {
    FullStable(usize, usize, usize),
    MajorMinor(usize, usize),
    MajorOnly(usize),
    NonStable(usize, usize, usize, usize, usize, Channel),
}

impl Version {
    /// If version is stable or not
    pub fn is_stable(&self) -> bool {
        match self {
            Self::NonStable(..) => false,
            _ => true,
        }
    }

    /// Returns the channel of the version string
    pub fn channel(&self) -> Channel {
        match self {
            Self::NonStable(.., c) => c.clone(),
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
