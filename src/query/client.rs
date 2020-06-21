use crate::assetmodel::AssetModelOwned;
use crate::errors::AmuriError;
use crate::level::LevelOwned;
use crate::version::Version;
use crate::traits::Retriever;
use serde::Deserialize;
use crate::snapshot_type::STMAP;
use crate::constants::REPO_ROOT_VAR;
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
    base_dir: String
}


fn get_latest(path: &std::path::Path) -> Result<String,AmuriError> {
    if !path.exists() {
        return Err(AmuriError::NonExtantPath(path.to_string_lossy().into_owned()));
    }
    
    if !path.is_dir() {
        return Err(AmuriError::NonExtantPath(path.to_string_lossy().into_owned()));
    }
    if path.read_dir().map_err(|e| AmuriError::IoError(e.to_string()))?.next().is_none() {
        return Err(AmuriError::EmptyDirectory(path.to_string_lossy().into_owned()))
    };
    let mut latest = String::new();
    for entry in std::fs::read_dir(&path).map_err(|e| AmuriError::IoError(e.to_string()))? {
        let entry = entry.map_err(|e| AmuriError::IoError(e.to_string()))?;
        let f_name = String::from(entry.file_name().to_string_lossy());
        if f_name == "current" {continue;}
        if f_name > latest {
            latest = f_name;
        }
    }
   return Ok(latest)
}

fn get_next(path: &std::path::Path) -> Result<String, AmuriError> {
    let latest = get_latest(path)?.parse::<u16>().unwrap();
    Ok(format!("{:04}", latest+1))

}

fn create_path(path: &std::path::Path) -> Result<(),AmuriError> {
    //println!("creating path {:?}",path);
    std::fs::create_dir_all(path)?;
    Ok(())
}

impl Retriever for Client {
    type AssetModelType = AssetModelOwned;
    type ErrorType = AmuriError;

    fn get(&self, asset_model: &Self::AssetModelType) -> Result<String, Self::ErrorType> {
        
        let mut asset_path = std::path::PathBuf::new();
        asset_path.push(self.get_root());

        match &asset_model.level {
            LevelOwned::Show(show) => asset_path.push(show),
            LevelOwned::Sequence { show, sequence } => {
                asset_path.push(show);
                asset_path.push(sequence)
            },
            LevelOwned::Shot {
                show,
                sequence,
                shot,
            } => {
                asset_path.push(show);
                asset_path.push(sequence);
                asset_path.push(shot);
            },
        }

        let extension = STMAP.get(&asset_model.snapshot_type).ok_or_else(|| AmuriError::UnknownSnapshotType(asset_model.snapshot_type.clone()))?;

        asset_path.push(&asset_model.name);
        asset_path.push(&asset_model.department);
        asset_path.push(&asset_model.subcontext);
        asset_path.push(&asset_model.snapshot_type);

        //println!("create missing? {:?}", asset_model.create_missing);
        // can probably get rid of this extra allocation by being a bit more clever in forming
        // the route next
        match asset_model.version.as_ref().unwrap_or(&Version::Current) {
            Version::Current => asset_path.push("current"),
            Version::Latest => {asset_path.push(get_latest(&asset_path)? )},
            Version::Next => { 
                let next_unchecked = get_next(&asset_path);
                match  next_unchecked {
                    Ok(c) => {
                        asset_path.push(c);
                        if asset_model.create_missing {
                            create_path(&asset_path)?
                        }
                    },
                    Err(AmuriError::NonExtantPath(_)) => {
                        asset_path.push("0001".to_string());
                        if asset_model.create_missing {
                            create_path(&asset_path)?;
                        }
                    }
                    Err(AmuriError::EmptyDirectory(_)) => {
                        asset_path.push("0001".to_string());
                        if asset_model.create_missing {
                            create_path(&asset_path)?;
                        }
                    },
                    _ => return next_unchecked,
                }
            },
            Version::Number(num) => asset_path.push(format!("{:04}", num)),
        }
        let main = "main".to_string();
        let key = asset_model.key.as_ref().unwrap_or(&main);
        let filename = format!("{}.{}", &key, &extension);
        asset_path.push(filename);

        Ok(asset_path.to_string_lossy().into_owned())
    }
    
}
impl Client {
    /// New up a client given the base directory
    pub fn new<I: Into<String>>(base_dir: I) -> Self { 
        Self {
            base_dir:  base_dir.into()  
        }
    }
    /// Generate server from env var. This should be the preferred
    /// way of instantiating the Client.
    pub fn from_env() -> Result<Self, AmuriError> {
        let root = env::var(REPO_ROOT_VAR)?;
        Ok(Self::new(root))
    }
    
    /// Retrieve the base directory
    pub fn get_root(&self) -> &str {
        self.base_dir.as_ref()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::assetmodel::AssetModel;

    fn setup() {
        std::env::set_var(REPO_ROOT_VAR, "/home/jgerber/src/rust/amuri/tests/data");
    }
    fn setup_tmp() {
        std::env::set_var(REPO_ROOT_VAR, "/tmp/amuri_tests");
    }

    #[test]
    fn can_get_latest_fn() {
        setup();
        let path = std::path::Path::new("/home/jgerber/src/rust/amuri/tests/data/TESTSHOW/robot/model/hi/maya_model");
        let latest = get_latest(&path);
        assert_eq!(latest, Ok("0004".to_string())) ;  
    }

    #[test]
    fn can_get_latest_from_client() {
        setup();
        let asset_model = AssetModel::from_strs(
            "asset", "TESTSHOW", "robot", "model", "hi", "maya_model", Some("latest"), Some("main"), "false"
        ).unwrap();

        let client = Client::from_env().unwrap();
        let results = client.get(&asset_model.to_owned());
        assert_eq!(results, Ok("/home/jgerber/src/rust/amuri/tests/data/TESTSHOW/robot/model/hi/maya_model/0004/main.mb".to_string())) ;  
    }

    #[test]
    fn can_get_next_from_client() {
        setup();
        let asset_model = AssetModel::from_strs(
            "asset", "TESTSHOW", "robot", "model", "hi", "maya_model", Some("next"), Some("main"), "false"
        ).unwrap();

        let client = Client::from_env().unwrap();
        let results = client.get(&asset_model.to_owned());
        assert_eq!(results, Ok("/home/jgerber/src/rust/amuri/tests/data/TESTSHOW/robot/model/hi/maya_model/0005/main.mb".to_string())) ;  
    }

    #[test]
    fn can_get_next_from_client_and_create_missing() {
        setup_tmp();
        let asset_model = AssetModel::from_strs(
            "asset", "TESTSHOW", "robot", "model", "hi", "maya_model", Some("next"), Some("main"), "true"
        ).unwrap();

        let client = Client::from_env().unwrap();
        let results = client.get(&asset_model.to_owned());
        let expected = "/tmp/amuri_tests/TESTSHOW/robot/model/hi/maya_model/0001/main.mb";
        assert_eq!(results, Ok(expected.to_string())) ;  
        let expected_path = std::path::Path::new("/tmp/amuri_tests/TESTSHOW/robot/model/hi/maya_model/0001");
        assert!(expected_path.exists());
        assert!(std::fs::remove_dir_all("/tmp/amuri_tests").is_ok());
    }


    #[test]
    fn can_get_version_from_client() {
        setup();
        let asset_model = AssetModel::from_strs(
            "asset", "TESTSHOW", "robot", "model", "hi", "maya_model", Some("4"), Some("main"), "false"
        ).unwrap();

        let client = Client::from_env().unwrap();
        let results = client.get(&asset_model.to_owned());
        assert_eq!(results, Ok("/home/jgerber/src/rust/amuri/tests/data/TESTSHOW/robot/model/hi/maya_model/0004/main.mb".to_string())) ;  
    }

    #[test]
    fn can_get_next_fn() {
        setup();
        let path = std::path::Path::new("/home/jgerber/src/rust/amuri/tests/data/TESTSHOW/robot/model/hi/maya_model");
        let next = get_next(&path);
        assert_eq!(next, Ok("0005".to_string())) ;  
    }
}