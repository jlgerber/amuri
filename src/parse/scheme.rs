use nom::{branch::alt, bytes::complete::tag, sequence::terminated, IResult};

/// Parse the scheme for the set of asset manager routes
pub fn scheme_parser(input: &str) -> IResult<&str, &str> {
    terminated(
        alt((tag("asset"), tag("instance"), tag("render"), tag("plate"))),
        tag(":"),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scheme_parser() {
        assert_eq!(scheme_parser("asset:"), Ok(("", "asset")));
        assert_eq!(scheme_parser("instance:"), Ok(("", "instance")));
        assert_eq!(scheme_parser("render:"), Ok(("", "render")));
        assert_eq!(scheme_parser("plate:"), Ok(("", "plate")));
        assert_eq!(scheme_parser("renderful:").is_err(), true);
        assert_eq!(scheme_parser("frender:").is_err(), true);
    }
}
