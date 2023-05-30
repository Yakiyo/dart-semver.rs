use std::str::FromStr;

#[derive(Debug, Default, PartialEq, Clone, Copy)]
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

impl std::convert::From<&str> for Channel {
    fn from(value: &str) -> Self {
        Channel::from_str(value).unwrap()
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;

    use super::Channel;

    #[test]
    fn channel_test() {
        assert_eq!(Channel::from_str("3.0.1"), Ok(Channel::Stable));
        assert_eq!(Channel::from_str("3.0.1-5.6.dev"), Ok(Channel::Dev));
        assert_eq!(Channel::from_str("3.0.1-5.6.beta"), Ok(Channel::Beta));
    }
}
