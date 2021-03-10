use serde_derive::Deserialize;
use tera::{Tera, Context};
use inflector::Inflector;

#[derive(Clone, Deserialize)]
pub struct SeedConfig {
    pub version: String,
    header: Option<String>,
    enum_header: Option<String>,
    pub enum_footer: Option<String>,
    pub enum_output: Option<String>,
    security: Generate,
    common: Generate,
    example: Generate,
}

#[derive(Clone, Deserialize)]
pub struct Generate {
    pub up_sql: String,
    pub down_sql: String,
    pub model: String,
}

impl SeedConfig{
    pub fn load() -> Result<SeedConfig, std::io::Error> {
        let cnt=std::fs::read_to_string("seed.toml")?;
        let config: SeedConfig = toml::from_str(cnt.as_str())?;
        Ok(config)
    }

    pub fn module_conf(&self, table: &str) -> Option<&Generate> {
        match table {
            "security" => Some(&self.security),
            "common" => Some(&self.common),
            "example" => Some(&self.example),
            _ => None
        }
    }

    pub fn get_header(&self, table: &str) -> String {
        let mut tera = Tera::default();
        let mut context = Context::new();

        context.insert("table", table);
        tera.render_str(self.header.as_ref().unwrap().as_str(), &context).unwrap()
    }

    pub fn get_enum_header(&self, module: &str) -> String {
        let mut tera = Tera::default();
        let mut context = Context::new();
        context.insert("module", module.to_title_case().as_str());
        tera.render_str(self.enum_header.as_ref().unwrap().as_str(), &context).unwrap()
    }
}

#[test]
fn toml_works() {
    let config: SeedConfig = toml::from_str(r#"
        version="0.1"

        [example]
        sql="migrations/2021-03-09-083808_example/up.sql"
        model="src/models/example.rs"
        header='''use serde_derive::{Deserialize, Serialize};
        use crate::schema::{{table}};
        use diesel::prelude::*;
        '''
    "#).unwrap();

    assert_eq!(config.version, "0.1");
}

#[test]
fn load_works() -> anyhow::Result<()> {
    let cnt=std::fs::read_to_string("seed.toml")?;
    let config: SeedConfig = toml::from_str(cnt.as_str())?;
    assert_eq!(config.version, "0.1");

    let cf=SeedConfig::load()?;
    assert_eq!(cf.version, "0.1");

    let header=cf.header.unwrap();
    println!("{}", header);

    let mut tera = Tera::default();
    let mut context = Context::new();
    context.insert("table", "example");
    let result = tera.render_str(header.as_str(), &context);
    println!("{}", result.unwrap());

    Ok(())
}
