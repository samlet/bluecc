use serde_derive::Deserialize;
use tera::{Tera, Context};

#[derive(Deserialize)]
struct SeedConfig {
    version: String,
    generate: Generate,
}

#[derive(Deserialize)]
struct Generate {
    sql: String,
    model: String,
    header: Option<String>,
}

impl SeedConfig{
    pub fn load() -> Result<SeedConfig, std::io::Error> {
        let cnt=std::fs::read_to_string("seed.toml")?;
        let config: SeedConfig = toml::from_str(cnt.as_str())?;
        Ok(config)
    }

    pub fn init(table: &str) -> anyhow::Result<Self> {
        let mut tera = Tera::default();
        let mut context = Context::new();
        context.insert("table", table);

        let mut cf=Self::load()?;
        let header=cf.generate.header.unwrap();
        let result = tera.render_str(header.as_str(), &context)?;
        cf.generate.header=Some(result);
        Ok(cf)
    }
}

#[test]
fn toml_works() {
    let config: SeedConfig = toml::from_str(r#"
        version="0.1"

        [generate]
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

    let header=cf.generate.header.unwrap();
    println!("{}", header);

    let mut tera = Tera::default();
    let mut context = Context::new();
    context.insert("table", "example");
    let result = tera.render_str(header.as_str(), &context);
    println!("{}", result.unwrap());

    Ok(())
}

