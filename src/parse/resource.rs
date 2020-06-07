use crate::parse::utils::*;
use nom::{character, error::ErrorKind, IResult};

/// Parse a component of a resource. This must start with a letter
/// contain zero or more valid body chars, and not end in an underscore
pub fn parse_resource(input: &str) -> IResult<&str, &str> {
    let (i, r1) = character::complete::alpha1(input)?;
    let (i, r2) = valid_body0_parser(i)?;
    if r2.len() > 0 {
        // This will be a bit slower, but will be utf8 compliant. We want a
        // slice that is guaranteed to consist of the last character of the str.
        // we need to figure out how big that last character is in utf8, and use
        // that info to create the slice.
        // This is probably overkill. TODO: look into whether a multibyte's
        // utf8 char's last byte could equal 0x5f and thus yield a false positive.
        // If not, then we can just index into the str slice directly and skip the following
        // line:
        let last_char_sz = r2.chars().last().unwrap().len_utf8();
        let (_, _) = character::complete::none_of::<_, _, (&str, ErrorKind)>("_")(
            &r2[r2.len() - last_char_sz..],
        )?;
    }
    Ok((i, &input[0..r1.len() + r2.len()]))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_parse_resource() {
        assert_eq!(parse_resource("a123b"), Ok(("", "a123b")));
        assert_eq!(parse_resource("a123_").is_err(), true);
        assert_eq!(parse_resource("a123b"), Ok(("", "a123b")));
    }
}
