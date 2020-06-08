use strum;
use thiserror::Error; //ParseError
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AmuriError {
    #[error("Unable to convert str {target} to int")]
    StrToIntError { target: String },
    #[error("Error parsing uri {cause:?}")]
    UriParsingError { cause: String },
    #[error("Error parsing scheme from str {cause:?}")]
    SchemeParsingError {
        #[from]
        cause: strum::ParseError,
    },
    #[error("Failed to parse level from {0}")]
    LevelParsingFailure(String),
}
