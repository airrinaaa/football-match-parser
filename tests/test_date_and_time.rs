use anyhow::Result;
use pest::Parser;
use football_match_parser::{MatchGrammar, Rule};

#[test]
fn valid_dates() -> Result<()> {
    let valid_dates = ["2025-11-05", "2000-01-03", "1976-03-30"];
    for date in valid_dates{
        let result = MatchGrammar::parse(Rule::date, date)?;
        assert_eq!(result.as_str(), date, "Should accept: {}", date);
    }
    Ok(())
}

#[test]
fn invalid_dates() {
    let invalid_dates = ["25-11-05", "2025/11/05", "20251105", "abcd-ef-gh", "1976-3-30", "1976-10-3", "2025-13-01", "2025-11-33"];
    for date in invalid_dates{
        let result = MatchGrammar::parse(Rule::date, date);
        assert!(result.is_err(), "Should reject: {}", date);
    }
}

#[test]
fn valid_time() -> Result<()> {
    let valid_time = ["00:00", "09:45", "23:59", "04:05", "17:19"];
    for time in valid_time{
        let result = MatchGrammar::parse(Rule::time, time)?;
        assert_eq!(result.as_str(), time, "Should accept: {}", time);
    }
    Ok(())
}

#[test]
fn invalid_time() {
    let invalid_time = ["0000", "24:61", "ab:cd", "5-30", "24:00", "13:99", "15:3"];
    for time in invalid_time{
        let result = MatchGrammar::parse(Rule::time, time);
        assert!(result.is_err(), "Should reject: {}", time);
    }
}
