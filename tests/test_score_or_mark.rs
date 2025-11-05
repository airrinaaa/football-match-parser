use pest::Parser;
use pest_derive::Parser;
use anyhow::{Result};

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
struct Grammar;
#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn valid_score_or_mark() -> Result<()> {
        let valid_score = ["2:1", "1:1", "LIVE", "live", "41'", "90+6'", "-"];
        for score in valid_score {
            let result = Grammar::parse(Rule::score_or_mark, score)?;
            assert_eq!(result.as_str(), score);
        }
        Ok(())
    }

    #[test]
    fn invalid_score_or_mark() -> Result<()> {
        let invalid_score = ["2-1", "liv", "45", "90+", ":", "234:1"];
        for score in invalid_score {
            let result = Grammar::parse(Rule::score_or_mark, score);
            assert!(result.is_err(), "Should reject: {}", score);
        }
        Ok(())
    }
}