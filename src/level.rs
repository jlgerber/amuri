//! Level enums
//!
//! The level comes in two flavors: owned and non-owned
//! It is not clear yet which i will use.
use crate::errors::AmuriError;
use crate::parse;
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum Level<'a> {
    Show(&'a str),
    Sequence {
        show: &'a str,
        sequence: &'a str,
    },
    Shot {
        show: &'a str,
        sequence: &'a str,
        shot: &'a str,
    },
}

impl<'a> Level<'a> {
    /// constructor function
    pub fn show(name: &'a str) -> Self {
        Level::Show(name)
    }

    /// constructor function
    pub fn seq(show: &'a str, sequence: &'a str) -> Self {
        Level::Sequence { show, sequence }
    }

    /// constructor function
    pub fn sequence(show: &'a str, sequence: &'a str) -> Self {
        Level::Sequence { show, sequence }
    }

    /// constructor function
    pub fn shot(show: &'a str, sequence: &'a str, shot: &'a str) -> Self {
        Level::Shot {
            show,
            sequence,
            shot,
        }
    }

    /// parse level from str.
    // Note: I did not use the trait due to lifetime conflicts with trait
    pub fn from_str(input: &'a str) -> std::result::Result<Self, AmuriError> {
        let (_, result) = parse::level::parse_level_noslash(input)
            .map_err(|_e| AmuriError::LevelParsingFailure(input.into()))?;
        Ok(result)
    }

    /// Create an OwnedLevel from a level
    pub fn to_owned(&self) -> OwnedLevel {
        match self {
            Self::Show(show) => OwnedLevel::show(*show),
            Self::Sequence { show, sequence } => OwnedLevel::seq(*show, *sequence),
            Self::Shot {
                show,
                sequence,
                shot,
            } => OwnedLevel::shot(*show, *sequence, *shot),
        }
    }
}
impl<'a> From<Level<'a>> for OwnedLevel {
    fn from(input: Level<'a>) -> Self {
        match input {
            Level::Show(show) => OwnedLevel::show(show),
            Level::Sequence { show, sequence } => OwnedLevel::seq(show, sequence),
            Level::Shot {
                show,
                sequence,
                shot,
            } => OwnedLevel::shot(show, sequence, shot),
        }
    }
}
#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone)]
pub enum OwnedLevel {
    Show(String),
    Sequence {
        show: String,
        sequence: String,
    },
    Shot {
        show: String,
        sequence: String,
        shot: String,
    },
}

impl OwnedLevel {
    /// Show constructor function
    pub fn show<I: Into<String>>(name: I) -> Self {
        OwnedLevel::Show(name.into())
    }

    /// Seq constructor function
    pub fn seq<I: Into<String>>(show: I, sequence: I) -> Self {
        OwnedLevel::Sequence {
            show: show.into(),
            sequence: sequence.into(),
        }
    }

    /// Sequence constructor function
    pub fn sequence<I: Into<String>>(show: I, sequence: I) -> Self {
        OwnedLevel::Sequence {
            show: show.into(),
            sequence: sequence.into(),
        }
    }

    /// Shot constructor function
    pub fn shot<I: Into<String>>(show: I, sequence: I, shot: I) -> Self {
        OwnedLevel::Shot {
            show: show.into(),
            sequence: sequence.into(),
            shot: shot.into(),
        }
    }

    /// Generate a levelspec String
    pub fn to_string(&self) -> String {
        match self {
            Self::Show(show) => show.to_string(),
            Self::Sequence { show, sequence } => format!("{}.{}", show, sequence),
            Self::Shot {
                show,
                sequence,
                shot,
            } => format!("{}.{}.{}", show, sequence, shot),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    mod level {
        use super::*;
        // use nom::error::ErrorKind;
        // use nom::Err::Error;
        #[test]
        fn can_construct_shot_from_str() {
            let level = Level::from_str("dev02.rd.9999");
            assert_eq!(level, Ok(Level::shot("dev02", "rd", "9999")));
        }
        #[test]
        fn can_construct_seq_from_str() {
            let level = Level::from_str("dev02.rd");
            assert_eq!(level, Ok(Level::seq("dev02", "rd")));
        }
        #[test]
        fn can_construct_show_from_str() {
            let level = Level::from_str("dev02");
            assert_eq!(level, Ok(Level::show("dev02")));
        }
        #[test]
        fn from_str_will_fail_with_invalid_input() {
            let level = Level::from_str("_dev02.rd.9999");
            assert_eq!(
                level,
                Err(AmuriError::LevelParsingFailure("_dev02.rd.9999".into()))
            );
        }
        #[test]
        fn can_convert_shot_level_to_ownedlevel() {
            let level = Level::from_str("dev02.rd.9999").unwrap();
            let owned = level.to_owned();
            assert_eq!(owned, OwnedLevel::shot("dev02", "rd", "9999"));
        }
        #[test]
        fn can_convert_seq_level_to_ownedlevel() {
            let level = Level::from_str("dev02.rd").unwrap();
            let owned = level.to_owned();
            assert_eq!(owned, OwnedLevel::seq("dev02", "rd"));
        }
        #[test]
        fn can_convert_show_level_to_ownedlevel() {
            let level = Level::from_str("dev02").unwrap();
            let owned = level.to_owned();
            assert_eq!(owned, OwnedLevel::show("dev02"));
        }
    }
}
