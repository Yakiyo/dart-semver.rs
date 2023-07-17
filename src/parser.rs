#![allow(clippy::needless_bool)]
use crate::channel::Channel;
use crate::version::Version;
use pest::error as perr;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammer.pest"]
pub struct VersionParser;

impl VersionParser {
    pub fn version<S: AsRef<str>>(s: S) -> Result<Version, Box<perr::Error<Rule>>> {
        let s = s.as_ref();

        let map: HashMap<Rule, String> = VersionParser::parse(Rule::version, s)?
            .flatten()
            .map(|f| (f.as_rule(), f.as_str().to_string()))
            .collect();

        let (ma, mi, pa, pr, prp) = (
            shorty(&map, &Rule::major),
            shorty(&map, &Rule::minor),
            shorty(&map, &Rule::patch),
            shorty(&map, &Rule::prerelease),
            shorty(&map, &Rule::prerelease_patch),
        );
        // if its a non-stable version
        if map.contains_key(&Rule::non_stable) {
            return Ok(Version::NonStable(
                ma.unwrap(),
                mi.unwrap(),
                pa.unwrap(),
                pr.unwrap(),
                prp.unwrap(),
                Channel::Dev,
            ));
        } else if map.contains_key(&Rule::full_stable) {
            return Ok(Version::FullStable(ma.unwrap(), mi.unwrap(), pa.unwrap()));
        } else if map.contains_key(&Rule::major_minor) {
            return Ok(Version::MajorMinor(ma.unwrap(), mi.unwrap()));
        } else if map.contains_key(&Rule::major_only) {
            return Ok(Version::MajorOnly(ma.unwrap()));
        }
        panic!("Unable to parse. Received {}", s);
    }
}

/// short internal func
fn shorty(map: &HashMap<Rule, String>, key: &Rule) -> Option<usize> {
    map.get(key)
        .map(|f| f.parse::<usize>().expect("Unable to convert str to usize"))
}

#[cfg(test)]
mod test {
    use super::VersionParser as vp;
    use super::*;

    #[test]
    fn parse_test() {
        vp::version("v3.4.5").unwrap();
        vp::version("3.4.5").unwrap();
        vp::version("3.4").unwrap();
        vp::version("3").unwrap();
        vp::version("v3.4.5-1.2.beta").unwrap();
        vp::version("3.4.5-1.2.beta").unwrap();
    }

    #[test]
    fn version_test() {
        assert_eq!(vp::version("v3.4.5").unwrap(), Version::FullStable(3, 4, 5));
        assert_eq!(vp::version("3.4").unwrap(), Version::MajorMinor(3, 4));
        assert_eq!(vp::version("v3").unwrap(), Version::MajorOnly(3));
        assert_eq!(
            vp::version("v3.4.5-1.2.dev").unwrap(),
            Version::NonStable(3, 4, 5, 1, 2, Channel::Dev)
        );
    }
}
