use super::container_name::parse_name_noslash;
use nom::IResult;

/// Retrieve the department name given an input str
pub fn parse_snapshot_type(input: &str) -> IResult<&str, &str> {
    parse_name_noslash(input)
}
