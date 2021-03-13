use serde_json::json;
use crate::GenericError;
use glob::{MatchOptions, glob_with};
use std::fs::read_to_string;
use std::path::PathBuf;
use thiserror::private::PathAsDisplay;
use std::str::FromStr;
use std::str;
use super::*;
use serde::{Serialize, de};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataFile{
    pub uri: String,
    pub path: String,
    pub content: String,
    #[serde(default)]
    pub entities: Vec<String>,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DataFiles{
    pub files: Vec<DataFile>,
}

pub fn list_files(dir: &str, pattern: &str) -> Result<Vec<PathBuf>, GenericError>{
    let options = MatchOptions {
        case_sensitive: false,
        ..Default::default()
    };

    let mut rs:Vec<PathBuf>=Vec::new();
    for entry in glob_with(
        format!("{}/{}", dir, pattern).as_str(), options)? {
        let path = entry?;
        // rs.push(path.to_str().unwrap().to_string());
        rs.push(path.to_owned());
    }
    Ok(rs)
}

#[test]
fn json_object_works() -> anyhow::Result<()> {
    use crate::models::security_types::SecurityGroup;
    let json = json!(
        {
          "groupId": 212118821551607808_i64,
          "groupName": "Full Admin",
          "description": "Full Admin group, has all general permissions."
        }
    );
    let rec = serde_json::from_value::<SecurityGroup>(json)?;
    println!("{:?}", rec);
    Ok(())
}

#[test]
fn list_files_works() -> anyhow::Result<()> {
    let files=list_files("./data", "**/*.xml")?;
    for f in &files{
        println!("{:?}: {}", f.file_name().unwrap().to_string_lossy(), f.as_display());
    }

    let f=files.get(0).unwrap();
    let cnt=std::fs::read_to_string(f)?;
    println!("{}", cnt);
    Ok(())
}

pub fn merge_files(dir: &str, filter: &str, json_output: &str)
    -> Result<String,GenericError>{

    use std::io::prelude::*;

    let mut data_files=DataFiles{ files: vec![] };
    let files=list_files(dir, filter)?;
    for f in &files{
        println!(".. read {} start", f.as_display());
        let cnt=std::fs::read_to_string(f)?;
        // println!(".. read {} end", f.as_display());
        let path=f.to_str().unwrap().to_owned();
        data_files.files.push(DataFile{
            uri: f.file_name().unwrap().to_str().unwrap().to_string(),
            path: path.to_owned(),
            content: cnt,
            entities: get_entities_in_file(path.as_str())?.iter()
                .map(|e|e.clone())
                .collect::<Vec<String>>(),
        });
    }

    let zout=store_z(&data_files, json_output)?;
    Ok(zout)
}

pub fn store_z<T>(data_files:&T, json_output: &str) -> Result<String,GenericError>
where
    T: ?Sized + Serialize,{
    use std::io::prelude::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;

    let val=serde_json::to_string_pretty(&data_files)?;
    std::fs::write(json_output, &val)?;

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&val.as_bytes());
    let compressed_bytes = e.finish()?;
    let zout=format!("{}z", json_output);
    std::fs::write(&zout, compressed_bytes)?;
    Ok(zout)
}

#[test]
fn merge_files_works() -> anyhow::Result<()> {
    let zout=merge_files("./data", "**/*.xml",
                "./.store/data_files.json")?;
    println!("save to {}", zout);
    Ok(())
}

pub fn load_z<T>(bytes:&[u8]) -> Result<T, GenericError>
where
    T: de::DeserializeOwned,{
    use std::io::prelude::*;
    use flate2::read::ZlibDecoder;

    let mut d = ZlibDecoder::new(bytes);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    info!("size {}", &s.len());
    let data_files:T=serde_json::from_str(&s)?;

    Ok(data_files)
}

#[test]
fn load_z_works() -> anyhow::Result<()> {
    let bytes:&[u8] =include_bytes!("data_files.z");
    let data_files=load_z::<DataFiles>(bytes)?;
    for f in &data_files.files{
        println!("{}: {}", f.uri, f.path);
    }
    Ok(())
}

#[test]
fn load_z_file_works() -> anyhow::Result<()> {
    let bytes =std::fs::read("./.store/data_files.jsonz")?;
    let data_files=load_z::<DataFiles>(&bytes)?;
    for f in &data_files.files{
        println!("{}: {}", f.uri, f.path);
        println!("\t{:?}", f.entities);
    }
    Ok(())
}

