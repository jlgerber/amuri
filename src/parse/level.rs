//! Simplified levelspec parser
//!
//! level may be
//! show
//! show.seq
//! show.seq.shot
//! show

use crate::level::Level;
use crate::parse::resource::parse_resource;
use crate::parse::utils::*;
use nom::branch::alt;
use nom::bytes::complete::tag;
use nom::sequence::{preceded, separated_pair, terminated};
use nom::{character, error::ErrorKind, IResult};

pub fn parse_level_item(input: &str) -> IResult<&str, &str> {
    let (i, r1) = character::complete::alphanumeric1(input)?;
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

/// given a valid show string, return a show and whatever is left over
pub fn parse_show(input: &str) -> IResult<&str, Level> {
    let (i, r) = parse_resource(input)?;
    Ok((i, Level::Show(r)))
}

/// given a valid seq str, return a Level::Seq and whatever is left over
pub fn parse_seq(input: &str) -> IResult<&str, Level> {
    let (i, (sh, seq)) = separated_pair(parse_resource, tag("."), parse_resource)(input)?;
    Ok((i, Level::seq(sh, seq)))
}
/// For those who prefer the full name
pub fn parse_sequence(input: &str) -> IResult<&str, Level> {
    parse_seq(input)
}
/// parse a shot returning a Level::Shot instance
pub fn parse_shot(input: &str) -> IResult<&str, Level> {
    let (i, (sh, seq)) = separated_pair(parse_resource, tag("."), parse_level_item)(input)?;
    let (i, shot) = preceded(tag("."), parse_level_item)(i)?;
    Ok((i, Level::shot(sh, seq, shot)))
}

/// Parse a simplified levelspec string, which may be show, show.seq, or show.seq.shot
pub fn parse_level(input: &str) -> IResult<&str, Level> {
    terminated(alt((parse_shot, parse_seq, parse_show)), tag("/"))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::error::ErrorKind;
    use nom::Err::Error;
    mod parse_show {
        use super::*;
        #[test]
        fn can_parse_show_starting_with_letter_and_ending_with_number() {
            // parses show name
            assert_eq!(parse_show("DEV01"), Ok(("", Level::show("DEV01"))));
        }
        #[test]
        fn can_parse_show_up_to_period() {
            // parses up to first period
            assert_eq!(parse_show("DEV01."), Ok((".", Level::show("DEV01"))));
        }
        #[test]
        fn cannot_parse_show_ending_with_underscore() {
            // cannot parse a name ending with an underscore
            assert_eq!(
                parse_show("DEV01_"),
                Err(nom::Err::Error(("01_", ErrorKind::Verify)))
            );
        }
    }

    mod parse_seq {
        use super::*;

        #[test]
        fn can_parse_seq() {
            // can pase a sequence
            assert_eq!(parse_seq("dev01.rd"), Ok(("", Level::seq("dev01", "rd"))));
        }
        #[test]
        fn can_parse_seq_up_to_period() {
            // can pase a sequence
            assert_eq!(parse_seq("dev01.rd."), Ok((".", Level::seq("dev01", "rd"))));
        }
    }
    mod level_item {
        use super::*;

        #[test]
        fn can_parse_level_item_consisting_of_numbers() {
            assert_eq!(parse_level_item("0001"), Ok(("", "0001")));
        }

        #[test]
        fn can_parse_level_item_consisting_of_letters_and_numbers() {
            assert_eq!(parse_level_item("a0001"), Ok(("", "a0001")));
        }

        #[test]
        fn can_parse_level_item_up_to_space() {
            assert_eq!(parse_level_item("a 0001"), Ok((" 0001", "a")));
        }

        #[test]
        fn cannot_parse_level_item_ending_in_underscore() {
            assert_eq!(
                parse_level_item("0001_"),
                Err(nom::Err::Error(("_", ErrorKind::NoneOf)))
            );
        }
    }
    mod parse_shot {
        use super::*;

        #[test]
        fn test_parse_shot() {
            assert_eq!(
                parse_shot("dev01.rd.0001"),
                Ok(("", Level::shot("dev01", "rd", "0001")))
            );
        }
    }
    mod parse_level {
        use super::*;

        #[test]
        fn can_parse_level_from_shot() {
            // shot
            assert_eq!(
                parse_level("dev01.rd.9999/"),
                Ok(("", Level::shot("dev01", "rd", "9999")))
            );
        }
        #[test]
        fn can_parse_level_from_shot_up_to_slash() {
            assert_eq!(
                parse_level("dev01.rd.9999/"),
                Ok(("", Level::shot("dev01", "rd", "9999")))
            );
        }
        #[test]
        fn can_parse_level_from_seq_up_to_slash() {
            assert_eq!(
                parse_level("dev01.rd/"),
                Ok(("", Level::seq("dev01", "rd")))
            );
        }
        #[test]
        fn can_parse_level_from_show_up_to_slash() {
            assert_eq!(parse_level("dev01/"), Ok(("", Level::show("dev01"))));
            // what happens if we throw in a non-supported char?
        }
        #[test]
        fn can_parse_level_from_seq_up_to_space() {
            assert_eq!(
                parse_level("dev01 .rd/"),
                Err(Error((" .rd/", ErrorKind::Tag)))
            );
        }
    }
}
