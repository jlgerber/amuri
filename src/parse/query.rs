use crate::parse::resource::parse_resource;
use nom::bytes::complete::tag;
use nom::multi::many0;
use nom::sequence::preceded;
use nom::IResult;
#[derive(Debug, PartialEq, Eq)]
pub struct QueryPair<'a> {
    pub key: &'a str,
    pub value: &'a str,
}

impl<'a> QueryPair<'a> {
    pub fn from(key: &'a str, value: &'a str) -> Self {
        Self { key, value }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct OwnedQueryPair {
    pub key: String,
    pub value: String,
}

impl<'a> From<QueryPair<'a>> for OwnedQueryPair {
    fn from(input: QueryPair<'a>) -> Self {
        Self {
            key: input.key.into(),
            value: input.value.into(),
        }
    }
}

pub fn parse_and<'a>(input: &'a str) -> IResult<&'a str, QueryPair<'a>> {
    let (i, key) = preceded(tag("&"), parse_resource)(input)?;
    let (i, value) = preceded(tag("="), parse_resource)(i)?;
    Ok((i, QueryPair::from(key, value)))
}

pub fn parse_query<'a>(input: &'a str) -> IResult<&'a str, Vec<QueryPair<'a>>> {
    let (i, key) = preceded(tag("?"), parse_resource)(input)?;
    let (i, value) = preceded(tag("="), parse_resource)(i)?;
    let (i, mut pairs) = many0(parse_and)(i)?;
    let mut rval: Vec<QueryPair> = Vec::with_capacity(pairs.len() + 1);
    rval.push(QueryPair::from(key, value));
    rval.append(&mut pairs);
    Ok((i, rval))
}

#[cfg(test)]
mod tests {
    use super::*;
    mod parse_query {
        use super::*;

        #[test]
        fn can_parse_query() {
            assert_eq!(
                parse_query("?version=current"),
                Ok(("", vec![QueryPair::from("version", "current")]))
            )
        }
        #[test]
        fn can_parse_query_and() {
            assert_eq!(
                parse_query("?version=current&server=organic"),
                Ok((
                    "",
                    vec![
                        QueryPair::from("version", "current"),
                        QueryPair::from("server", "organic")
                    ]
                ))
            )
        }
    }
    mod parse_and {
        use super::*;

        #[test]
        fn can_parse_and() {
            assert_eq!(
                parse_and("&version=current"),
                Ok(("", QueryPair::from("version", "current")))
            )
        }
    }
}
