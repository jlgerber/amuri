use crate::errors::AmuriError;
use crate::level::OwnedLevel;
use crate::scheme::Scheme;

#[derive(Debug, PartialEq, Eq)]
pub enum Version {
    Current,
    Latest,
    Number(u16),
}

impl Version {
    /// Generate Version from str
    pub fn from_str(input: &str) -> Result<Self, AmuriError> {
        match input {
            "current" => Ok(Self::Current),
            "latest" => Ok(Self::Latest),
            _ => {
                let num: u16 = input.parse().map_err(|_x| AmuriError::StrToIntError {
                    target: input.into(),
                })?;
                Ok(Self::Number(num))
            }
        }
    }
}
/// Represents the query
#[derive(Debug, PartialEq, Eq)]
pub struct AssetModel {
    container_type: Scheme,
    level: OwnedLevel,
    department: String,
    subcontext: String,
    snapshot_type: String,
    version: Version,
    /// The particular filetype we are interested in
    key: Option<String>,
}

#[cfg(test)]
mod tests {
    use super::*;
    mod version {
        use super::*;
        #[test]
        fn can_generate_version_from_str_current() {
            assert_eq!(Version::from_str("current"), Ok(Version::Current));
        }

        #[test]
        fn can_generate_version_from_str_latest() {
            assert_eq!(Version::from_str("latest"), Ok(Version::Latest));
        }

        #[test]
        fn can_generate_version_from_str_number() {
            assert_eq!(Version::from_str("0001"), Ok(Version::Number(1)));
        }
        #[test]
        fn will_return_error_if_given_non_numeric_str() {
            assert_eq!(
                Version::from_str("fred"),
                Err(AmuriError::StrToIntError {
                    target: "fred".into()
                })
            );
        }
    }
}
