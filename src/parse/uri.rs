//! Uri provides the entrypoint parsing function, used to transform a str into
//! an AssetModel
use crate::assetmodel::AssetModel;
use crate::errors::AmuriError;
use crate::parse::{
    container_name::parse_name, department::parse_department, hashkey::parse_hashtag,
    level::parse_level, query::parse_query, scheme::parse_scheme,
    snapshot_type::parse_snapshot_type, subcontext::parse_subcontext,
};
use crate::version::Version;
use nom::combinator::{all_consuming, opt};
use nom::sequence::tuple;
/// parse hashtag from str, which should generally take the form
/// ```#key```
pub fn parse_uri(input: &str) -> Result<AssetModel, crate::errors::AmuriError> {
    let (_i, (scheme, level, name, dept, subcontext, snaptype, query, key)) =
        all_consuming(tuple((
            parse_scheme,
            parse_level,
            parse_name,
            parse_department,
            parse_subcontext,
            parse_snapshot_type,
            parse_query,
            opt(parse_hashtag),
        )))(input)
        .map_err(|err| AmuriError::UriParsingError {
            cause: format!("{:?}", err),
        })?;
    let mut version = None;
    for querypair in query {
        if querypair.key == "version" {
            version = Some(Version::from_str(querypair.value)?);
            break;
        }
    }
    Ok(AssetModel::new(
        scheme, level, name, dept, subcontext, snaptype, version, key,
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_parse_uri() {
        let uri = parse_uri("asset://dev01/bob/model/hi/maya_model?version=current#main");
        let expect = AssetModel::from_strs(
            "asset",
            "dev01",
            "bob",
            "model",
            "hi",
            "maya_model",
            Some("current"),
            Some("main"),
        );
        assert_eq!(uri, expect);
    }
}
