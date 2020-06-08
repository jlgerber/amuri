use super::container_name::parse_name;
use nom::IResult;

/// Retrieve the department name given an input str
pub fn parse_department(input: &str) -> IResult<&str, &str> {
    parse_name(input)
}
