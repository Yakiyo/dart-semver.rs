use pest::error as perr;
use pest::Parser;
use pest_derive::Parser;
use std::collections::HashMap;

#[derive(Parser)]
#[grammar = "grammer.pest"]
pub struct VersionParser;

impl VersionParser {
    #[allow(unused)]
    pub fn version<S: AsRef<str>>(s: S) -> Result<HashMap<Rule, String>, Box<perr::Error<Rule>>> {
        let s = s.as_ref();
        let flattened: HashMap<Rule, String> = VersionParser::parse(Rule::version, s)?
            .flatten()
            .map(|f| (f.as_rule(), f.as_str().to_string()))
            .collect();
        
        Ok(flattened)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let res = VersionParser::version("3.4.5-3.4.beta").unwrap();
        println!("{res:?}");
    }
}
