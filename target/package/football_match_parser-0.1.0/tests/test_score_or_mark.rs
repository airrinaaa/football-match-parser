use anyhow::Result;
use football_match_parser::{MatchGrammar, Rule};
use pest::Parser;

#[test]
fn valid_score_or_mark() -> Result<()> {
    let valid_score = [
        "2:1", "1:1", "LIVE", "live", "41'", "90+6'", "-", "2 :  1", "45 '", "90  +3'",
    ];
    for score in valid_score {
        let result = MatchGrammar::parse(Rule::score_or_mark, score)?;
        assert_eq!(result.as_str(), score);
    }
    Ok(())
}

#[test]
fn invalid_score_or_mark() {
    let invalid_score = ["2-1", "liv", "45", "90+", ":", "234:1"];
    for score in invalid_score {
        let result = MatchGrammar::parse(Rule::score_or_mark, score);
        assert!(result.is_err(), "Should reject: {}", score);
    }
}
