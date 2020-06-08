use crate::parse::resource::parse_resource;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair};
use nom::{character, error::ErrorKind, IResult};

/// parse hashtag from str, which should generally take the form
/// ```#key```
pub fn parse_hashtag(input: &str) -> IResult<&str, &str> {
    preceded(tag("#"), parse_resource)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err::Error;

    #[test]
    fn can_parse_hashtag() {
        assert_eq!(parse_hashtag("#main"), Ok(("", "main")))
    }
    #[test]
    fn cannot_start_with_number() {
        assert_eq!(
            parse_hashtag("#1main"),
            Err(Error(("1main", ErrorKind::Alpha)))
        );
    }
    #[test]
    fn cannot_parse_a_symbol() {
        assert_eq!(parse_hashtag("#ma$in"), Ok(("$in", "ma")));
    }
    #[test]
    fn cannot_end_with_underscore() {
        assert_eq!(
            parse_hashtag("#main_"),
            Err(Error(("_", ErrorKind::Verify)))
        );
    }
}
