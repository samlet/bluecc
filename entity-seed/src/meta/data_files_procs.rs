use serde_json::json;
use crate::GenericError;
use glob::{MatchOptions, glob_with};
use std::fs::read_to_string;
use std::path::PathBuf;
use thiserror::private::PathAsDisplay;
use std::str::FromStr;
use std::str;

#[derive(Debug, Deserialize, Serialize, Clone)]
struct DataFile{
    pub uri: String,
    pub path: String,
    pub content: String,
}
#[derive(Debug, Deserialize, Serialize, Clone)]
struct DataFiles{
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

#[test]
fn merge_files_works() -> anyhow::Result<()> {
    use std::io::prelude::*;
    use flate2::Compression;
    use flate2::write::ZlibEncoder;

    let mut data_files=DataFiles{ files: vec![] };
    let files=list_files("./data", "**/*.xml")?;
    for f in &files{
        println!(".. read {} start", f.as_display());
        let cnt=std::fs::read_to_string(f)?;
        // println!(".. read {} end", f.as_display());
        data_files.files.push(DataFile{
            uri: f.file_name().unwrap().to_str().unwrap().to_string(),
            path: f.to_str().unwrap().to_string(),
            content: cnt,
        });
    }

    let val=serde_json::to_string_pretty(&data_files)?;
    std::fs::write("./.store/data_files.json", &val)?;

    let mut e = ZlibEncoder::new(Vec::new(), Compression::default());
    e.write_all(&val.as_bytes());
    let compressed_bytes = e.finish()?;
    std::fs::write("./.store/data_files.z", compressed_bytes)?;

    Ok(())
}

#[test]
fn load_z_works() -> anyhow::Result<()> {
    use std::io::prelude::*;
    use flate2::read::ZlibDecoder;
    let bytes:&[u8] =include_bytes!("data_files.z");
    let mut d = ZlibDecoder::new(bytes);
    let mut s = String::new();
    d.read_to_string(&mut s).unwrap();
    println!("size {}", s.len());

    let data_files:DataFiles=serde_json::from_str(&s)?;
    for f in &data_files.files{
        println!("{}: {}", f.uri, f.path);
    }
    Ok(())
}