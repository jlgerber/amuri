//! parse the name from the uri
//!
//! Given the assumption that we are here:
//! scheme:level/name

use crate::parse::resource::parse_resource;
use crate::parse::utils::*;
use nom::bytes::complete::tag;
use nom::sequence::terminated;
use nom::{character, combinator::verify, IResult};

fn parse_asset_name_(input: &str) -> IResult<&str, &str> {
    let (i, r) = verify(parse_resource, |s: &str| {
        !character::is_digit(s.chars().last().unwrap_or('1') as u8)
    })(input)?;
    Ok((i, r))
}

/// given a valid show string, return a show and whatever is left over
pub fn parse_asset_name(input: &str) -> IResult<&str, &str> {
    terminated(parse_asset_name_, tag("/"))(input)
}

fn parse_instance_name_(input: &str) -> IResult<&str, &str> {
    let (i, r1) = character::complete::alpha1(input)?;
    let (i, r2) = verify(valid_body0_parser, |s: &str| {
        character::is_digit(s.chars().last().unwrap_or('1') as u8)
    })(i)?;
    Ok((i, &input[0..r1.len() + r2.len()]))
}

/// given a valid show string, return a show and whatever is left over
pub fn parse_instance_name(input: &str) -> IResult<&str, &str> {
    terminated(parse_instance_name_, tag("/"))(input)
}

/// parse name not ending in a slash
pub(crate) fn parse_name_noslash(input: &str) -> IResult<&str, &str> {
    let (i, r1) = character::complete::alpha1(input)?;
    let (i, r2) = verify(valid_body0_parser, |s: &str| {
        !is_underscore(s.chars().last().unwrap_or('1') as u8)
    })(i)?;
    Ok((i, &input[0..r1.len() + r2.len()]))
}

/// A less retrictive parser that doesnt care whether the last character is a
/// number or a letter.
pub fn parse_name(input: &str) -> IResult<&str, &str> {
    terminated(parse_name_noslash, tag("/"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err::Error;

    mod parse_asset_name {
        use super::*;
        #[test]
        pub fn name_can_start_and_end_with_letter() {
            // if the name starts and ends with a letter, we are good
            assert_eq!(parse_asset_name("fred/"), Ok(("", "fred")));
        }
        #[test]
        pub fn cannot_end_with_underscore() {
            // but we cannot end in an underscore
            assert_eq!(
                parse_asset_name("fred_/"),
                Err(Error(("_/", ErrorKind::Verify)))
            );
        }
        #[test]
        pub fn cannot_start_with_underscore() {
            // but we cannot end in an underscore
            assert_eq!(
                parse_asset_name("_fred/"),
                Err(Error(("_fred/", ErrorKind::Alpha)))
            );
        }
        #[test]
        pub fn cannot_parse_asset_name_ending_in_number() {
            // cannot parse an asset name with a number on the end
            assert_eq!(
                parse_asset_name("fred1/"),
                Err(Error(("fred1/", ErrorKind::Verify)))
            );
        }
    }
    mod parse_instance_name {
        use super::*;

        #[test]
        fn can_parse_name_ending_in_number() {
            assert_eq!(parse_instance_name("fred1/"), Ok(("", "fred1")));
        }
        #[test]
        fn can_have_internal_underscores() {
            // can have internal underscores
            assert_eq!(
                parse_instance_name("drop_dead_fred_1/"),
                Ok(("", "drop_dead_fred_1"))
            );
        }
        #[test]
        fn can_have_capital_letters() {
            // caps are not a problem
            assert_eq!(parse_instance_name("FRED1/"), Ok(("", "FRED1")));
        }
        #[test]
        fn cannot_start_with_a_number() {
            // must start with a letter
            assert_eq!(
                parse_instance_name("1fred1/"),
                Err(Error(("1fred1/", ErrorKind::Alpha)))
            );
        }
        #[test]
        fn cannot_end_with_a_letter() {
            // must end with a number
            assert_eq!(
                parse_instance_name("1fred/"),
                Err(Error(("1fred/", ErrorKind::Alpha)))
            );
        }
        #[test]
        fn cannot_end_with_an_underscore() {
            assert_eq!(
                parse_instance_name("fred_/"),
                Err(Error(("_/", ErrorKind::Verify)))
            );
        }
        #[test]
        fn cannot_start_with_an_underscore() {
            assert_eq!(
                parse_instance_name("_fred1/"),
                Err(Error(("_fred1/", ErrorKind::Alpha)))
            );
        }
        #[test]
        fn can_parse_instances_with_internal_underscores() {
            assert_eq!(
                parse_instance_name("fred_flinstone1/"),
                Ok(("", "fred_flinstone1"))
            );
        }
    }
    mod parse_name {
        use super::*;
        #[test]
        fn can_parse_name_which_starts_and_ends_with_letter() {
            assert_eq!(parse_name("fred/"), Ok(("", "fred")));
        }

        #[test]
        fn can_parse_name_which_starts_and_ends_with_uppercase_letter() {
            assert_eq!(parse_name("FRED/"), Ok(("", "FRED")));
        }

        #[test]
        fn can_parse_name_which_starts_with_letter_and_ends_with_number() {
            assert_eq!(parse_name("fred1/"), Ok(("", "fred1")));
        }
        #[test]
        fn can_parse_name_with_internal_underscores() {
            assert_eq!(parse_name("fred_flinstone/"), Ok(("", "fred_flinstone")));
        }

        #[test]
        fn fails_name_that_ends_in_underscore() {
            assert_eq!(
                parse_name("fred1_/"),
                Err(Error(("1_/", ErrorKind::Verify)))
            );
        }

        #[test]
        fn fails_name_that_starts_with_number() {
            assert_eq!(
                parse_name("1fred1/"),
                Err(Error(("1fred1/", ErrorKind::Alpha)))
            );
        }

        #[test]
        fn fails_name_that_starts_with_undercsore() {
            assert_eq!(
                parse_name("_fred1/"),
                Err(Error(("_fred1/", ErrorKind::Alpha)))
            );
        }
    }
}
