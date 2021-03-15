use serde::{Deserialize, Serialize};
use toml;

#[derive(Debug, Deserialize, PartialEq)]
pub struct Revisions {
    behaves: Vec<Behave>,
}
#[derive(Debug, Deserialize, PartialEq)]
pub enum Behave {
    RenameField { entity: String, from: String, to: String },
    DefaultTime { entity: String, field: String},
}

impl Revisions {
    pub fn get_field_rev(&self, ent: &str, fld: &str) -> String {
        for behave in &self.behaves {
            match behave {
                Behave::RenameField { entity, from, to } => {
                    if entity == ent && fld == from {
                        return to.to_owned();
                    }
                }
                _ => ()
            }
        }
        fld.to_string()
    }
}

#[test]
fn revs_works() -> anyhow::Result<()> {
    let toml_str = r#"
        behaves = [
            { RenameField = { entity = "CountryCapital", from = "countryCapital", to = "countryCapitalName" } },
            { RenameField = { entity = "CountryCode", from = "countryCode", to = "countryCodeId" } }
        ]"#;
    let revs:Revisions=toml::from_str(toml_str)?;
    println!("{:?}", revs);
    assert_eq!("countryCapitalName", revs.get_field_rev("CountryCapital", "countryCapital"));

    Ok(())
}

#[test]
fn from_inline_tables() {
    #[derive(Debug, Deserialize, PartialEq)]
    struct Multi {
        enums: Vec<TheEnum>,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    struct Val {
        val: TheEnum,
    }
    #[derive(Debug, Deserialize, PartialEq)]
    enum TheEnum {
        Plain,
        Tuple(i64, bool),
        NewType(String),
        Struct { value: i64 },
    }

    let toml_str = r#"
        enums = [
            { Plain = {} },
            { Tuple = { 0 = -123, 1 = true } },
            { NewType = "value" },
            { Struct = { value = -123 } }
        ]"#;
    assert_eq!(
        Multi {
            enums: vec![
                TheEnum::Plain,
                TheEnum::Tuple(-123, true),
                TheEnum::NewType("value".to_string()),
                TheEnum::Struct { value: -123 },
            ]
        },
        toml::from_str(toml_str).unwrap()
    );
}