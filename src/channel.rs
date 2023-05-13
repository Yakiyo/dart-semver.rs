#[derive(Debug, Default)]
pub enum Channel {
    #[default]
    Stable,
    Beta,
    Dev,
}

impl Channel {
    /// Returns a string version of the channel
    pub fn to_str(&self) -> &'static str {
        match self {
            Channel::Stable => "stable",
            Channel::Beta => "beta",
            Channel::Dev => "dev",
        }
    }
}

impl std::fmt::Display for Channel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_str())
    }
}

impl std::str::FromStr for Channel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let s = s.to_lowercase();
        if s.ends_with("beta") {
            Ok(Channel::Beta)
        } else if s.ends_with("dev") {
            Ok(Channel::Dev)
        } else {
            Ok(Channel::Stable)
        }
    }
}
