use pest_derive::Parser;
use thiserror::Error;

#[derive(Parser)]
#[grammar = "src/grammar.pest"]
pub struct MatchGrammar;

#[derive(Error, Debug)]
pub enum ParseError {
    #[error("Syntax error while parsing line: {0}")]
    Syntax(String),
    #[error("Invalid field format: {0}")]
    FieldFormat(String),
}