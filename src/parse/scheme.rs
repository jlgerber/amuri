use crate::scheme::Scheme;
use nom::{branch::alt, bytes::complete::tag, sequence::terminated, IResult};
use std::str::FromStr;

/// Parse the scheme for the set of asset manager routes
pub fn scheme_parser(input: &str) -> IResult<&str, &str> {
    terminated(
        alt((tag("asset"), tag("instance"), tag("render"), tag("plate"))),
        tag("://"),
    )(input)
}

/// Generate a Scheme instance from an input string
pub fn parse_scheme(input: &str) -> IResult<&str, Scheme> {
    let (i, r) = scheme_parser(input)?;
    // should be able to unwrap here because we have successfully parsed
    // the scheme
    Ok((i, Scheme::from_str(r).unwrap()))
}
#[cfg(test)]
mod tests {
    use super::*;
    mod scheme_parser {
        use super::*;

        #[test]
        fn can_parse_asset_instance_render_plate_up_to_colon() {
            assert_eq!(scheme_parser("asset://"), Ok(("", "asset")));
            assert_eq!(scheme_parser("instance://"), Ok(("", "instance")));
            assert_eq!(scheme_parser("render://"), Ok(("", "render")));
            assert_eq!(scheme_parser("plate://"), Ok(("", "plate")));
        }
        #[test]
        fn other_schemes_are_not_supported() {
            assert_eq!(scheme_parser("renderful://").is_err(), true);
            assert_eq!(scheme_parser("frender://").is_err(), true);
        }
    }
}
