use anyhow::Result;
use football_match_parser::{MatchGrammar, Rule};
use pest::Parser;

#[test]
fn valid_match_lines() -> Result<()> {
    let valid_match_lines = [
        "2025-10-26; 17:15; RealMadrid - Barcelona; 2:1; Santiago  Bernabeu",
        "2025-11-04; 20:00; PSG   - Bavaria; LIVE; Parc des Princes",
        "2025-11-04; 20:00; PSG - Bavaria; 15'; Parc des   Princes",
        "2025-11-04; 20:00; PSG - Bavaria; live;   Parc des Princes; ongoing",
        "2025-12-06; 18:00; Kudrivka - Dynamo Kyiv; -   ; scheduled",
        "2025-11-04; 20:00; PSG - Bavaria; LIVE",
    ];
    for match_line in valid_match_lines {
        let result = MatchGrammar::parse(Rule::match_line, match_line)?;
        assert_eq!(result.as_str(), match_line, "Should accept: {}", match_line);
    }
    Ok(())
}

#[test]
fn invalid_match_lines() {
    let invalid_lines = [
        "",
        "2025-13-26; 17:15; RealMadrid - Barcelona; 2:1",
        "2025-10-26; 25:00; PSG - Bavaria; LIVE",
        "2025-10-26 17:15; PSG - Bavaria; 2:1",
        "2025-12-06; 18:00; Kudrivka - Dynamo Kyiv; - ; scheduled;",
        "2025-10-26; 17:15; PSG - Bavaria; LIVE; ; ;  scheduled",
        "2025-10-26; 17:15; PSG - Bavaria; LIVE; Camp*Nou",
    ];
    for line in invalid_lines {
        let result = MatchGrammar::parse(Rule::match_line, line);
        assert!(result.is_err(), "Should reject: {}", line.escape_default());
    }
}
