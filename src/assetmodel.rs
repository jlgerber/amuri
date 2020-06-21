use crate::errors::AmuriError;
use crate::level::{Level, LevelOwned};
use crate::scheme::Scheme;
use crate::version::Version;
use std::str::FromStr;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssetModel<'a> {
    pub container_type: Scheme,
    pub level: Level<'a>,
    pub name: &'a str,
    pub department: &'a str,
    pub subcontext: &'a str,
    pub snapshot_type: &'a str,
    pub version: Option<Version>,
    /// The particular filetype we are interested in
    pub key: Option<&'a str>,
    pub create_missing: bool
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
        create_missing: bool
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
            create_missing,
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
        create_missing: &'a str,
    ) -> std::result::Result<AssetModel<'a>, AmuriError> {
        let container_type = Scheme::from_str(container_type)?;
        let level = Level::from_str(level)?;
        let version = if version.is_some() {
            Some(Version::from_str(version.unwrap())?)
        } else {
            None
        };
        
        let create_missing = if create_missing == "true" {true} else {false};

        Ok(AssetModel::new(
            container_type,
            level,
            name,
            department,
            subcontext,
            snapshot_type,
            version,
            key,
            create_missing,
        ))
    }

    pub fn to_owned(&self) -> AssetModelOwned {
        AssetModelOwned::from(self)
    }

}
/// Represents the query
#[derive(Debug, PartialEq, Eq, Clone)]
pub struct AssetModelOwned {
    pub container_type: Scheme,
    pub level: LevelOwned,
    pub name: String,
    pub department: String,
    pub subcontext: String,
    pub snapshot_type: String,
    pub version: Option<Version>,
    /// The particular filetype we are interested in
    pub key: Option<String>,
    pub create_missing: bool,
}

impl<'a> From<AssetModel<'a>> for AssetModelOwned {
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
            create_missing: input.create_missing
        }
    }
}

impl<'a> From<&AssetModel<'a>> for AssetModelOwned {
    fn from(input: &AssetModel<'a>) -> Self {
        Self {
            container_type: input.container_type.clone(),
            level: input.level.to_owned(),
            name: input.name.into(),
            department: input.department.into(),
            subcontext: input.subcontext.into(),
            snapshot_type: input.snapshot_type.into(),
            version: input.version.clone(),
            key: input.key.map(str::to_string),
            create_missing: input.create_missing
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
            "false"
        );
        assert!(am.is_ok());
    }
}
