use anyhow::Result;
use football_match_parser::{MatchGrammar, Rule};
use pest::Parser;

#[test]
fn valid_team_names() -> Result<()> {
    let valid_team_names = [
        "PSG",
        "Real Madrid",
        "Paris Saint-Germain",
        "Atletico Madrid (B)",
        "Queen's Park",
        "St. Pauli",
        "Dagenham & Redbridge",
        "Metalist 1925 Kharkiv",
        "Dnipro-1",
        "U19",
        "Ukraine U-19",
    ];
    for team_name in valid_team_names {
        let result = MatchGrammar::parse(Rule::team_name, team_name)?;
        assert_eq!(result.as_str(), team_name, "Should accept: {}", team_name);
    }
    Ok(())
}

#[test]
fn invalid_team_names() {
    let invalid_team_names = [
        "Atletico Madrid-",
        "Real_Madrid",
        "Dynamo, Kyiv",
        "AC*Roma",
        "AC/Roma",
        "",
    ];
    for team_name in invalid_team_names {
        let result = MatchGrammar::parse(Rule::team_name, team_name);
        assert!(result.is_err(), "Should reject: {}", team_name);
    }
}

#[test]
fn valid_teams() -> Result<()> {
    let valid_teams = [
        "PSG - Bavaria",
        "Kudrivka -    Dynamo Kyiv",
        "Paris Saint-Germain   - Olympique Lyonnais",
        "Brighton & Hove Albion - Reading",
        "Atletico Madrid (B) - Rayo Vallecano",
        "Queen's Park - St. Pauli",
        "FC Barcelona - Real Madrid",
    ];
    for teams in valid_teams {
        let result = MatchGrammar::parse(Rule::teams, teams)?;
        assert_eq!(result.as_str(), teams, "Should accept: {}", teams);
    }
    Ok(())
}

#[test]
fn invalid_teams() {
    let invalid_teams = [
        "Real Madrid Barcelona",
        "- Barcelona",
        "Real_Madrid -",
        "Kudrivka & Dynamo Kyiv",
        "Dynamo\tKyiv-Epicentr",
        "Real Madrid — Barcelona",
        "Kudrivka# - Dynamo Kyiv",
        "Kudrivka-Dynamo Kyiv",
        "Kudrivka - Dynamo Kyiv-",
    ];
    for teams in invalid_teams {
        let result = MatchGrammar::parse(Rule::teams, teams);
        assert!(result.is_err(), "Should reject: {}", teams);
    }
}
#[test]
fn valid_team_word() -> Result<()> {
    for word in [
        "PSG",
        "Saint-Germain",
        "U19",
        "Dnipro-1",
        "Paris-Saint-Germain",
    ] {
        let result = MatchGrammar::parse(Rule::team_word, word).expect("parse should succeed");
        assert_eq!(result.as_str(), word);
    }
    Ok(())
}

#[test]
fn invalid_team_word() -> Result<()> {
    for word in ["U 19", "AC/Roma", "Kudrivka#"] {
        let result = MatchGrammar::parse(Rule::team_word, word);
        assert!(
            result.is_err() || result.unwrap().as_str() != word,
            "Should reject: {}",
            word
        );
    }
    Ok(())
}

#[test]
fn valid_name_char() -> Result<()> {
    for char in ["A", "z", "0", "(", ")", ".", "&", "'"] {
        let result = MatchGrammar::parse(Rule::name_char, char).expect("parse should succeed");
        assert_eq!(result.as_str(), char);
    }
    Ok(())
}

#[test]
fn invalid_name_char() -> Result<()> {
    for char in ["_", ",", "+", "/", "!", "@", "#", "?", ":", ";"] {
        let result = MatchGrammar::parse(Rule::name_char, char);
        assert!(result.is_err(), "Should reject: {}", char);
    }
    Ok(())
}
