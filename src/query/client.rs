use crate::assetmodel::AssetModel;
use crate::constants::*;
use crate::errors::AmuriError;
use crate::level::Level;
use crate::scheme::Scheme;
use crate::version::Version;
use serde::Deserialize;

use reqwest::blocking;
use std::env;

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct FileRecord {
    file_type: String,
    source_path: String,
}

#[derive(Debug, PartialEq, Eq, Deserialize)]
pub struct Files {
    files: Vec<FileRecord>,
}

pub struct Client {
    server: String,
    port: u32,
    api_version: u32,
}

impl Client {
    pub fn new<I: Into<String>>(server: I, port: u32, api_version: u32) -> Self {
        Self {
            server: server.into(),
            port,
            api_version,
        }
    }
    /// Generate server from env or defaults
    pub fn from_env() -> Self {
        let server = env::var(SERVER_VAR).unwrap_or(DEFAULT_SERVER.into());
        let port = env::var(PORT_VAR)
            .map(|v| v.parse::<u32>().unwrap_or(DEFAULT_PORT))
            .unwrap_or(DEFAULT_PORT);
        let api_version = env::var(API_VAR)
            .map(|v| v.parse::<u32>().unwrap_or(DEFAULT_API))
            .unwrap_or(DEFAULT_API);
        Self {
            server,
            port,
            api_version,
        }
    }

    // construct a route from a relative path
    fn baseroute(&self) -> String {
        format!(
            "http://{}:{}/api/v{}/",
            self.server, self.port, self.api_version
        )
    }

    /// Retrieve the path
    pub fn get(&self, asset_model: AssetModel) -> Result<String, AmuriError> {
        let (show, level) = match asset_model.level {
            Level::Show(show) => (show, "".to_string()),
            Level::Sequence { show, sequence } => (show, format!("+sequence:{}", sequence)),
            Level::Shot {
                show,
                sequence,
                shot,
            } => (show, format!("+shot:{}{}", sequence, shot)),
        };

        let name = match asset_model.container_type {
            Scheme::Asset => "name",
            Scheme::Instance => "instance_name",
            Scheme::Render => "render_name",
            Scheme::Plate => "plate_name",
        };

        // can probably get rid of this extra allocation by being a bit more clever in forming
        // the route next
        let version = match asset_model.version.unwrap_or(Version::Current) {
            Version::Current => "is_current:true".into(),
            Version::Latest => "is_latest:true".into(),
            Version::Number(num) => format!("version:{}", num),
        };

        //todo: context type
        let route = format!(
            "{}snapshots?project={}&query={}:{}{}+department:{}+subcontext:{}+snapshot_type:{}+{}&fields=files",
            self.baseroute(),
            show,
            name,
            asset_model.name,
            level,
            asset_model.department,
            asset_model.subcontext,
            asset_model.snapshot_type,
            version,
        );

        let json: Vec<Files> = blocking::get(&route)
            .map_err(|e| AmuriError::ReqwestError {
                route: route.clone(),
                error: format!("{:?}", e),
            })?
            .json()
            .map_err(|e| AmuriError::ReqwestJsonError {
                route: route.clone(),
                error: format!("{:?}", e),
            })?;

        let key = asset_model.key.unwrap_or("main");
        if json.len() == 0 {
            return Err(AmuriError::EmptyResponseError);
        }

        for file in &json[0].files {
            if file.file_type == key {
                return Ok(file.source_path.clone());
            }
        }
        Err(AmuriError::ReqwestResponseMissingKeyError(key.into()))
    }
}
