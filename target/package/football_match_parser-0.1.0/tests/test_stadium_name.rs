use anyhow::Result;
use football_match_parser::{MatchGrammar, Rule};
use pest::Parser;
#[test]
fn valid_stadium_names() -> Result<()> {
    let valid_stadiums = [
        "Santiago Bernabeu",
        "Parc des Princes",
        "St. Mary's Stadium",
        "Vorskla Stadium (Poltava)",
        "NSC Olimpiyskiy",
        "Dnipro Arena",
        "Signal Iduna Park (Dortmund)",
        "Queen's Park Arena",
        "Obolon Arena",
        "Goodison Park",
        "Stade de France",
        "Lviv Arena",
    ];
    for stadium in valid_stadiums {
        let result = MatchGrammar::parse(Rule::stadium_name, stadium)?;
        assert_eq!(result.as_str(), stadium, "Should accept: {}", stadium);
    }
    Ok(())
}

#[test]
fn invalid_stadium_names() {
    let invalid_stadiums = [
        "",
        " ",
        "Camp*Nou",
        "Old, Trafford",
        "National Arena+",
        "Stadion\nKyiv",
    ];
    for stadium in invalid_stadiums {
        let result = MatchGrammar::parse(Rule::stadium_name, stadium);
        assert!(result.is_err(), "Should reject: {}", stadium);
    }
}
