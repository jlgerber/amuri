use thiserror::Error;

#[derive(Error, Debug, PartialEq, Eq)]
pub enum AmuriError {
    #[error("Unable to convert str {target} to int")]
    StrToIntError { target: String },
}
