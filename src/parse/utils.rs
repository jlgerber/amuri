use nom::{character, error::ErrorKind, error::ParseError, IResult};
use nom::{AsChar, InputTakeAtPosition};

/// Tests if byte is ascii '_'
pub fn is_underscore(chr: u8) -> bool {
    chr == 0x5f
}

/// Tests if byte is a valid ascii character for the body of a query.
/// that is, either alphabetic, digit, or underscore
pub fn is_valid_body_char(chr: u8) -> bool {
    character::is_alphabetic(chr) || character::is_digit(chr) || is_underscore(chr)
}

// parses 1 or more valid body chars stream
fn valid_body1<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position1_complete(
        |item| !is_valid_body_char(item.as_char() as u8),
        ErrorKind::Alpha,
    )
}

// parses 0 or mre valid body chars in a stream
fn valid_body0<T, E: ParseError<T>>(input: T) -> IResult<T, T, E>
where
    T: InputTakeAtPosition,
    <T as InputTakeAtPosition>::Item: AsChar,
{
    input.split_at_position_complete(|item| !is_valid_body_char(item.as_char() as u8))
}

/// parse one or more chars in a supplied str and return the portion that
/// is alpha numeric or underscore
pub fn valid_body1_parser(input: &str) -> IResult<&str, &str> {
    valid_body1(input)
}

/// parse zero or more chars in a supplied str and return the portion that
/// is alpha numeric or underscore
pub fn valid_body0_parser(input: &str) -> IResult<&str, &str> {
    valid_body0(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::Err::Error;
    mod is_underscore {
        use super::*;

        #[test]
        fn can_parse_undercore() {
            assert!(is_underscore(b'_'));
        }
        #[test]
        fn will_reject_non_underscore() {
            assert_eq!(is_underscore(b'a'), false);
        }
    }

    mod is_valid_body_char {
        use super::*;

        #[test]
        fn can_parse_valid_chars_ie_letters_numbers_underscore() {
            let valid = [b'a', b'b', b'c', b'0', b'9', b'_'];
            for test in &valid {
                assert!(is_valid_body_char(*test));
            }
        }
        #[test]
        fn rejects_invalid_chars() {
            let invalid = [b'-', b'$', b'!', b' ', b',', b'.', b'/', b'?', b'\\', b'|'];
            for test in &invalid {
                assert_eq!(is_valid_body_char(*test), false);
            }
        }
    }

    mod valid_body1_parser {
        use super::*;

        #[test]
        fn can_parse_valid_body1_data() {
            assert_eq!(valid_body1_parser("abc_123-"), Ok(("-", "abc_123")));
        }
        #[test]
        fn will_fail_when_starting_with_invalid_char() {
            assert_eq!(
                valid_body1_parser("-abc_123-"),
                Err(Error(("-abc_123-", nom::error::ErrorKind::Alpha)))
            );
        }
        #[test]
        fn will_fail_if_presented_with_empty_string() {
            assert_eq!(
                valid_body1_parser(""),
                Err(Error(("", nom::error::ErrorKind::Alpha)))
            );
        }
    }
    mod valid_body0_parser {
        use super::*;

        #[test]
        fn can_parse_str_with_valid_chars() {
            assert_eq!(valid_body0_parser("abc_123-"), Ok(("-", "abc_123")));
        }
        #[test]
        fn will_make_no_progress_on_str_starting_with_invalid_char() {
            assert_eq!(valid_body0_parser("-abc_123-"), Ok(("-abc_123-", "")));
        }
        #[test]
        fn can_handle_empty_str() {
            assert_eq!(valid_body0_parser(""), Ok(("", "")));
        }
    }
}
