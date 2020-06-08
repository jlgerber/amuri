use super::container_name::parse_name;
use nom::IResult;

/// Retrieve the context name given an input str
pub fn parse_subcontext(input: &str) -> IResult<&str, &str> {
    parse_name(input)
}
