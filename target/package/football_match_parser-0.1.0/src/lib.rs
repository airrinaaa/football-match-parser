use chrono::NaiveDate;
use pest::Parser;
use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
pub struct MatchGrammar;

#[derive(Debug, PartialEq)]
pub struct Match {
    pub date: String,
    pub time: String,
    pub home_team: String,
    pub away_team: String,
    pub home_score: Option<u8>,
    pub away_score: Option<u8>,
    pub stadium: Option<String>,
    pub status: Option<String>,
}

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Syntax error while parsing line: {0}")]
    Syntax(String),
    #[error("Invalid field format: {0}")]
    FieldFormat(String),
    #[error("Logical inconsistency: {0}")]
    Logical(String),
}

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
