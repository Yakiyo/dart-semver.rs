use crate::{Channel, Version};
use pest::{error as perror, Parser};
use pest_derive::Parser;

#[derive(Parser)]
#[grammar = "grammer.pest"]
pub struct VersionParser;

impl VersionParser {
    /// parse a string to a `Version` struct
    pub fn version<S: AsRef<str>>(s: S) -> Result<Version, Box<perror::Error<Rule>>> {
        let s = s.as_ref();
        let mut version = Version {
            channel: Channel::Stable,
            major: 0,
            minor: 0,
            patch: 0,
            prerelease: None,
            prerelease_patch: None,
        };
        let parsed = VersionParser::parse(Rule::version, s)
            .map_err(Box::from)?
            .flatten();
        for pair in parsed {
            match pair.as_rule() {
                Rule::channel => version.channel = pair.as_str().parse().unwrap(),
                Rule::major => version.major = pair.as_str().parse().unwrap(),
                Rule::minor => version.minor = pair.as_str().parse().unwrap(),
                Rule::patch => version.patch = pair.as_str().parse().unwrap(),
                Rule::prerelease => version.prerelease = pair.as_str().parse::<usize>().ok(),
                Rule::prerelease_patch => {
                    version.prerelease_patch = pair.as_str().parse::<usize>().ok()
                }
                _ => {}
            }
        }
        Ok(version)
    }
}

#[cfg(test)]
mod test {
    use super::VersionParser;
    use super::{Channel, Version};
    use crate::parser::Rule;
    use pest::Parser;

    #[test]
    fn parse_test() {
        let stable = VersionParser::parse(Rule::version, "5.4.3");
        let dev = VersionParser::parse(Rule::version, "5.4.3-4.6.dev");
        let invalid = VersionParser::parse(Rule::version, "a.b-dev.c");
        assert!(stable.is_ok());
        assert!(dev.is_ok());
        assert!(invalid.is_err());
    }

    #[test]
    fn version_test() {
        let version = VersionParser::version("3.4.5").unwrap();
        let non_stable = VersionParser::version("3.4.5-6.7.dev").unwrap();
        assert_eq!(
            version,
            Version {
                channel: Channel::Stable,
                major: 3,
                minor: 4,
                patch: 5,
                prerelease: None,
                prerelease_patch: None
            }
        );
        assert_eq!(
            non_stable,
            Version {
                channel: Channel::Dev,
                major: 3,
                minor: 4,
                patch: 5,
                prerelease: Some(6),
                prerelease_patch: Some(7),
            }
        );
        assert!(version.is_stable());
        assert!(!non_stable.is_stable());
        println!("{} {}", version, non_stable);
    }
}
