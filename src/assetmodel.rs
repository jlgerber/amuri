use crate::errors::AmuriError;
use crate::level::{Level, OwnedLevel};
use crate::scheme::Scheme;
use crate::version::Version;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssetModel<'a> {
    container_type: Scheme,
    level: Level<'a>,
    name: &'a str,
    department: &'a str,
    subcontext: &'a str,
    snapshot_type: &'a str,
    version: Option<Version>,
    /// The particular filetype we are interested in
    key: Option<&'a str>,
}

impl<'a> AssetModel<'a> {
    pub fn new(
        container_type: Scheme,
        level: Level<'a>,
        name: &'a str,
        department: &'a str,
        subcontext: &'a str,
        snapshot_type: &'a str,
        version: Option<Version>,
        key: Option<&'a str>,
    ) -> Self {
        Self {
            container_type,
            level,
            name,
            department,
            subcontext,
            snapshot_type,
            version,
            /// The particular filetype we are interested in
            key,
        }
    }

    pub fn from_strs(
        container_type: &'a str,
        level: &'a str,
        name: &'a str,
        department: &'a str,
        subcontext: &'a str,
        snapshot_type: &'a str,
        version: Option<&'a str>,
        key: Option<&'a str>,
    ) -> std::result::Result<AssetModel<'a>, AmuriError> {
        let container_type = Scheme::from_str(container_type)?;
        let level = Level::from_str(level)?;
        let version = if version.is_some() {
            Some(Version::from_str(version.unwrap())?)
        } else {
            None
        };
        Ok(AssetModel::new(
            container_type,
            level,
            name,
            department,
            subcontext,
            snapshot_type,
            version,
            key,
        ))
    }
}
/// Represents the query
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct OwnedAssetModel {
    container_type: Scheme,
    level: OwnedLevel,
    name: String,
    department: String,
    subcontext: String,
    snapshot_type: String,
    version: Option<Version>,
    /// The particular filetype we are interested in
    key: Option<String>,
}

impl<'a> From<AssetModel<'a>> for OwnedAssetModel {
    fn from(input: AssetModel<'a>) -> Self {
        Self {
            container_type: input.container_type.clone(),
            level: input.level.into(),
            name: input.name.into(),
            department: input.department.into(),
            subcontext: input.subcontext.into(),
            snapshot_type: input.snapshot_type.into(),
            version: input.version.clone(),
            key: input.key.map(str::to_string),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn can_create_from_strs() {
        let am = AssetModel::from_strs(
            "asset",
            "dev01",
            "bob",
            "model",
            "hi",
            "alembic_model",
            Some("current"),
            Some("main"),
        );
        assert!(am.is_ok());
    }
}
