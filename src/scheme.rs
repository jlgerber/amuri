use strum_macros::{AsRefStr, Display, EnumIter, EnumString, IntoStaticStr};

#[derive(Debug, Display, PartialEq, Eq, AsRefStr, EnumString, EnumIter, IntoStaticStr)]
#[strum(serialize_all = "snake_case")]
pub enum Scheme {
    Asset,
    Instance,
    Render,
    Plate,
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::str::FromStr;
    use strum::IntoEnumIterator;

    #[test]
    fn test_can_display() {
        let tests = ["asset", "instance", "render", "plate"];
        for (cnt, scheme) in Scheme::iter().enumerate() {
            assert_eq!(&format!("{}", scheme), tests[cnt]);
        }
    }

    #[test]
    fn test_can_convert_to_ref() {
        let tests = ["asset", "instance", "render", "plate"];
        for (cnt, scheme) in Scheme::iter().enumerate() {
            assert_eq!(scheme.as_ref(), tests[cnt]);
        }
    }

    #[test]
    fn test_create_from_str() {
        let tests = ["asset", "instance", "render", "plate"];
        for cnt in 0..tests.len() {
            assert_eq!(Scheme::from_str(tests[cnt]).is_ok(), true);
        }
    }
}
