use anyhow::Result;
use pest::Parser;
use football_match_parser::{MatchGrammar, Rule};

#[test]
fn valid_statuses() -> Result<()> {
    let valid_statuses = [
        "played",
        "Played",
        "PLAYED",
        "scheduled",
        "Scheduled",
        "SCHEDULED",
        "ongoing",
        "OngoIng",
        "ONGOING",
    ];
    for status in valid_statuses{
        let result = MatchGrammar::parse(Rule::status, status)?;
        assert_eq!(result.as_str(), status, "Should accept: {}", status);
    }
    Ok(())
}

#[test]
fn invalid_statuses() {
    let invalid_statuses = [
        "playing",
        "delayed",
        "postponed",
        "Live",
        "live",
        "",
        "finished",
        "unknown",
    ];
    for status in invalid_statuses{
        let result = MatchGrammar::parse(Rule::status, status);
        assert!(result.is_err(), "Should reject: {}", status);
    }
}
