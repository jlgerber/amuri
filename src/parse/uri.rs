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
use nom::error::ErrorKind;
use nom::sequence::tuple;
use nom::Err::Error;

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
            opt(parse_query),
            opt(parse_hashtag),
        )))(input)
        .map_err(|err| match err {
            Error((errstr, ErrorKind::Alpha)) if errstr.len() > 0 => {
                let err_char = errstr.chars().next();
                let problem = match err_char {
                    Some(s) => s.to_string(),
                    None => "".into(),
                };
                let location = &input[..(input.len() - errstr.len() + problem.len())];
                AmuriError::UriNonAlphaParsingError {
                    problem,
                    location: location.into(),
                }
            }
            Error((errstr, ErrorKind::Verify)) if errstr.len() > 0 => {
                let processed = &input[..(input.len() - errstr.len())];
                AmuriError::UriUnexpectedUnderscoreParsingError {
                    processed: processed.into(),
                    remaining: errstr.into(),
                }
            }
            //
            _ => AmuriError::UriParsingError {
                cause: format!("{:?}", err),
            },
        })?;

    let mut version = None;
    let mut create_missing = false;
    if query.is_some() {
        for querypair in query.unwrap() {
            if querypair.key == "version" {
                version = Some(Version::from_str(querypair.value)?);
            }
            if querypair.key == "create" {
                create_missing = querypair.value.parse::<bool>().unwrap_or(false);
            }
        }
    }
    Ok(AssetModel::new(
        scheme, level, name, dept, subcontext, snaptype, version, key, create_missing
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_parse_asset_uri_with_version_and_key() {
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
            "false"
        );
        assert_eq!(uri, expect);
    }
    #[test]
    fn can_parse_asset_uri_with_version_and_key_and_create() {
        let uri = parse_uri("asset://dev01/bob/model/hi/maya_model?version=current&create=true#main");
        let expect = AssetModel::from_strs(
            "asset",
            "dev01",
            "bob",
            "model",
            "hi",
            "maya_model",
            Some("current"),
            Some("main"),
            "true"
        );
        assert_eq!(uri, expect);
    }
    #[test]
    fn can_parse_asset_uri_with_num_version_and_key() {
        let uri = parse_uri("asset://dev01/bob/model/hi/maya_model?version=1#main");
        let expect = AssetModel::from_strs(
            "asset",
            "dev01",
            "bob",
            "model",
            "hi",
            "maya_model",
            Some("1"),
            Some("main"),
            "false"
        );
        assert_eq!(uri, expect);
    }

    #[test]
    fn can_parse_asset_uri_with_version_no_key() {
        let uri = parse_uri("asset://dev01/bob/model/hi/maya_model?version=current");
        let expect = AssetModel::from_strs(
            "asset",
            "dev01",
            "bob",
            "model",
            "hi",
            "maya_model",
            Some("current"),
            None,
            "false"
        );
        assert_eq!(uri, expect);
    }

    #[test]
    fn can_parse_asset_uri_with_no_version_no_key() {
        let uri = parse_uri("asset://dev01/bob/model/hi/maya_model");
        let expect = AssetModel::from_strs(
            "asset",
            "dev01",
            "bob",
            "model",
            "hi",
            "maya_model",
            None,
            None,
            "false"
        );
        assert_eq!(uri, expect);
    }
    #[test]
    fn can_handle_bad_scheme() {
        let uri = parse_uri("assetf://dev01/bob/model/hi/maya_model");
        let expect = Err(AmuriError::UriParsingError {
            cause: "Error((\"f://dev01/bob/model/hi/maya_model\", Tag))".to_string(),
        });
        assert_eq!(uri, expect);
    }
}
