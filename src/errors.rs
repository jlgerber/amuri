use strum;
use thiserror::Error; //ParseError
#[derive(Error, Debug, PartialEq, Eq)]
pub enum AmuriError {
    #[error("Unable to convert str {target} to int")]
    StrToIntError { target: String },
    #[error("Error parsing uri.\nOriginal parser error: '{cause:?}'")]
    UriParsingError { cause: String },
    #[error("Error parsing scheme from str {cause:?}")]
    SchemeParsingError {
        #[from]
        cause: strum::ParseError,
    },
    #[error("Failed to parse level from {0}")]
    LevelParsingFailure(String),
    #[error("Failed to parse uri. Problem character encountered.\nExpected a-z|A-Z but found: '{problem}'\nParsing haulted here: '{location}'")]
    UriNonAlphaParsingError { problem: String, location: String },
    #[error("Failed to parse uri. Segment ending in '_' where not allowed.\nProcessed:'{processed}'\nRemaining: '{remaining}'")]
    UriUnexpectedUnderscoreParsingError {
        processed: String,
        remaining: String,
    },
    #[error("request failed: Route: {route}, error: {error}")]
    ReqwestError { route: String, error: String },
    #[error("request failed to deserialize response to json: Route: {route}, error: {error}")]
    ReqwestJsonError { route: String, error: String },
    #[error("Response missing key: {0}")]
    ReqwestResponseMissingKeyError(String),
    #[error("Empty response")]
    EmptyResponseError,
}
