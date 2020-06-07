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
    #[test]
    fn test_is_underscore() {
        assert!(is_underscore(b'_'));
        assert_eq!(is_underscore(b'a'), false);
    }

    #[test]
    fn test_is_valid_body_char() {
        let valid = [b'a', b'b', b'c', b'0', b'9', b'_'];
        for test in &valid {
            assert!(is_valid_body_char(*test));
        }
        let invalid = [b'-', b'$', b'!', b' ', b',', b'.', b'/', b'?', b'\\', b'|'];
        for test in &invalid {
            assert_eq!(is_valid_body_char(*test), false);
        }
    }

    #[test]
    fn test_valid_body1_parser() {
        assert_eq!(valid_body1_parser("abc_123-"), Ok(("-", "abc_123")));
        assert_eq!(
            valid_body1_parser("-abc_123-"),
            Err(Error(("-abc_123-", nom::error::ErrorKind::Alpha)))
        );
    }

    #[test]
    fn test_valid_body0_parser() {
        assert_eq!(valid_body0_parser("abc_123-"), Ok(("-", "abc_123")));
        assert_eq!(valid_body0_parser("-abc_123-"), Ok(("-abc_123-", "")));
    }
}
