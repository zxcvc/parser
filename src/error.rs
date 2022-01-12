use super::parser::ast::error::ParseError;
use super::scanner::error::ScanError;

#[derive(Debug, Clone)]
pub struct NoContentError {
    pub code: i32,
    pub describe: String,
}
impl NoContentError {
    pub fn new() -> Self {
        Self {
            code: 500,
            describe: "Unexpected end of input".to_string(),
        }
    }
}

impl From<ScanError> for ParseError {
    fn from(error: ScanError) -> Self {
        Self {
            code: 400,
            describe: error.describe,
            position: error.position,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SyntaxError {
    ScanError(ScanError),
    ParseError(ParseError),
    NoContentError(NoContentError),
}

impl From<ScanError> for SyntaxError {
    fn from(error: ScanError) -> Self {
        Self::ScanError(error)
    }
}

impl From<ParseError> for SyntaxError {
    fn from(error: ParseError) -> Self {
        Self::ParseError(error)
    }
}

impl From<NoContentError> for SyntaxError {
    fn from(error: NoContentError) -> Self {
        Self::NoContentError(error)
    }
}
