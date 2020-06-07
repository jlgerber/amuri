use crate::parse::utils::*;
use nom::{character, combinator::verify, IResult};

/// Parse a component of a resource. This must start with a letter
/// contain zero or more valid body chars, and not end in an underscore
pub fn parse_resource(input: &str) -> IResult<&str, &str> {
    let (i, r1) = character::complete::alpha1(input)?;
    let (i, r2) = verify(valid_body0_parser, |s: &str| {
        s.chars().last().unwrap_or('a') != '_'
    })(i)?;
    Ok((i, &input[0..r1.len() + r2.len()]))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err::Error;
    #[test]
    fn test_parse_resource() {
        assert_eq!(parse_resource("a123b"), Ok(("", "a123b")));
        assert_eq!(parse_resource("a123b"), Ok(("", "a123b")));
    }

    #[test]
    fn test_parse_resource_ending_in_underscore() {
        assert_eq!(
            parse_resource("a123_"),
            Err(Error(("123_", ErrorKind::Verify)))
        );
    }
}
