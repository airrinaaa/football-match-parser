use chrono::NaiveDate;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

///Parser for football match lines based on the grammar defined in grammar.pest
#[derive(Parser)]
#[grammar = "src/grammar.pest"]
pub struct MatchGrammar;

///Represents one football match with parsed data from the input line
#[derive(Debug, PartialEq)]
pub struct Match {
    ///The date of the match in YYYY-MM-DD format
    pub date: String,
    /// The match start time in HH:MM format
    pub time: String,
    ///The home team name
    pub home_team: String,
    ///The away team name
    pub away_team: String,
    ///The home team's score(if the match has already been played)
    pub home_score: Option<u8>,
    ///The away team's score(if the match has already been played)
    pub away_score: Option<u8>,
    ///The stadium where the match is taking place
    pub stadium: Option<String>,
    ///The current status of the match(played, ongoing, scheduled)
    pub status: Option<String>,
}

///Custom error type
#[derive(Error, Debug)]
pub enum ParseError {
    ///Syntax - when a line doesn't match grammar rules  
    #[error("Syntax error while parsing line: {0}")]
    Syntax(String),
    ///FieldFormat - when a field has an invalid format(for ex. date doesn't exist)
    #[error("Invalid field format: {0}")]
    FieldFormat(String),
    ///Logical - when parsed data is logically inconsistent(for ex. wrong match status)
    #[error("Logical inconsistency: {0}")]
    Logical(String),
}

///Parses one line of football match data into a Match struct.
///Each line should follow the format:
///YYYY-MM-DD; HH:MM; HomeTeam - AwayTeam; ScoreOrMark; Stadium(optional); Status(optional)
///It returns:
///Ok(Match) if parsing succeeds  
///Err(ParseError) if syntax, format, or logical errors occur
pub fn parse_match_line(line: &str) -> Result<Match, ParseError> {
    let pairs = MatchGrammar::parse(Rule::match_line, line)
        .map_err(|_| ParseError::Syntax(format!("Invalid syntax in line: {}", line)))?;

    let mut date = String::new();
    let mut time = String::new();
    let mut home_team = String::new();
    let mut away_team = String::new();
    let mut home_score: Option<u8> = None;
    let mut away_score: Option<u8> = None;
    let mut stadium: Option<String> = None;
    let mut status: Option<String> = None;
    let mut expected_status: Option<String> = None;

    for pair in pairs.flatten() {
        match pair.as_rule() {
            Rule::date => {
                let field = pair.as_str().trim();
                NaiveDate::parse_from_str(field, "%Y-%m-%d")
                    .map_err(|_| ParseError::FieldFormat(format!("Invalid date: {}", field)))?;
                date = field.to_string();
            }

            Rule::time => {
                let field = pair.as_str().trim();
                time = field.to_string();
            }

            Rule::teams => {
                let parts: Vec<&str> = pair.as_str().split('-').collect();
                if parts.len() != 2 {
                    return Err(ParseError::Syntax(format!(
                        "Invalid teams format: {}",
                        pair.as_str()
                    )));
                }
                home_team = parts[0].trim().to_string();
                away_team = parts[1].trim().to_string();
            }

            Rule::score_or_mark => {
                let field = pair.as_str().trim().to_lowercase();

                if let Some((home, away)) = field.split_once(':') {
                    home_score = Some(home.trim().parse().map_err(|_| {
                        ParseError::FieldFormat(format!("Invalid home score: {}", home.trim()))
                    })?);

                    away_score = Some(away.trim().parse().map_err(|_| {
                        ParseError::FieldFormat(format!("Invalid away score: {}", away.trim()))
                    })?);
                    expected_status = Some("played".to_string());
                } else if field == "-" {
                    expected_status = Some("scheduled".to_string());
                } else if field == "live" || field.ends_with('\'') {
                    expected_status = Some("ongoing".to_string());
                } else {
                    return Err(ParseError::FieldFormat(format!(
                        "Unknown score or mark: {}",
                        field
                    )));
                }
            }

            Rule::stadium_name => {
                let field = pair.as_str().trim().to_string();
                let lower = field.to_lowercase();
                let reserved_statuses = ["played", "scheduled", "ongoing", "live"];
                if reserved_statuses.contains(&lower.as_str()) {
                    status = Some(lower);
                } else if !field.is_empty() {
                    stadium = Some(field);
                }
            }

            Rule::status => {
                if status.is_none() {
                    status = Some(pair.as_str().trim().to_lowercase());
                }
            }
            _ => {}
        }
    }
    if status.is_none() {
        status = expected_status.clone();
    }
    validate_logical_mismatch(status.as_deref(), expected_status.as_deref())?;

    Ok(Match {
        date,
        time,
        home_team,
        away_team,
        home_score,
        away_score,
        stadium,
        status,
    })
}
///Validates whether the detected match status matches the expected one.
fn validate_logical_mismatch(
    status: Option<&str>,
    expected_status: Option<&str>,
) -> Result<(), ParseError> {
    match (expected_status, status) {
        (None, None) => Err(ParseError::Logical(
            "Cannot determine match status".to_string(),
        )),
        (Some(expected), Some(actual)) if expected != actual => Err(ParseError::Logical(format!(
            "Status mismatch: expected '{}', but found '{}'",
            expected, actual
        ))),
        _ => Ok(()),
    }
}
