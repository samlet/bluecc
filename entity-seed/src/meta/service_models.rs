use std::str;
use serde_xml_rs::from_str;
use decimal::prelude::*;

use crate::meta_model::*;

#[derive(Debug, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

// ------

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceModel{
    pub description: String,
    pub vendor: String,
    pub version: Decimal,
    #[serde(rename = "service", default)]
    pub services: Vec<ModelService>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelService{
    pub name: String,
    #[serde(rename = "default-entity-name", default)]
    pub default_entity_name: String,
    pub engine: String,
    #[serde(default)]
    pub invoke: String,
    #[serde(default)]
    pub location: String,
    #[serde(default)]
    pub auth: bool,
    #[serde(default)]
    pub export: bool,
    #[serde(default)]
    pub validate: bool,
    #[serde(default)]
    pub description: String,

    pub implements: Option<ServiceImplements>,
    #[serde(rename = "permission-service", default)]
    pub permission_service: Option<ModelPermission>,
    #[serde(rename = "auto-attributes", default)]
    pub auto_attributes: Vec<ServiceAutoAttributes>,
    #[serde(rename = "override", default)]
    pub overrides: Vec<ServiceOverride>,
    #[serde(rename = "attribute", default)]
    pub attributes: Vec<ServiceAttribute>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "kebab-case")]
pub struct ModelPermission{
    pub service_name: String,
    pub main_action: String,
    #[serde(default)]
    pub resource_description: String,
    #[serde(default)]
    pub require_new_transaction: bool
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceImplements{
    pub service: String
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceAutoAttributes{
    pub include: String,
    #[serde(default)]
    pub mode: String,
    #[serde(rename = "entity-name", default)]
    pub entity_name: String,
    #[serde(default)]
    pub optional: bool
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceOverride{
    #[serde(default)]
    pub mode: String,
    pub name: String,
    #[serde(default)]
    pub optional: bool
}
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ServiceAttribute{
    #[serde(rename = "type", default)]
    pub data_type: String,
    pub mode: String,
    pub name: String,
    #[serde(default)]
    pub optional: bool
}

fn ex_service_models() -> ServiceModel{
    // from_reader(include_bytes!("fixtures/entitymodel_example.xml").unwrap()).unwrap()
    from_str(str::from_utf8(include_bytes!("fixtures/services.xml")).unwrap()).unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::meta::cc_conf::CcConfig;

    /// https://github.com/paupino/rust-decimal/blob/master/tests/decimal_tests.rs
    #[test]
    fn it_can_mulassign() {
        let mut a = Decimal::from_str("1.25").unwrap();
        let b = Decimal::from_str("0.01").unwrap();

        a *= b;
        assert_eq!("0.0125", a.to_string());

        a *= &b;
        assert_eq!("0.000125", a.to_string());

        let mut c = &mut a;
        c *= b;
        assert_eq!("0.00000125", a.to_string());

        let mut c = &mut a;
        c *= &b;
        assert_eq!("0.0000000125", a.to_string());
    }

    #[test]
    fn simple_struct_from_attribute_and_child() {
        let _ = simple_logger::init();

        let s = r##"
        <item name="hello">
            <source>world.rs</source>
        </item>
    "##;

        let item: Item = from_str(s).unwrap();

        assert_eq!(
            item,
            Item {
                name: "hello".to_string(),
                source: "world.rs".to_string(),
            }
        );

    }

    #[test]
    fn service_model_works() {
        let model:ServiceModel=ex_service_models();
        println!("{}", model.version);
        assert_eq!("1.0", model.version.to_string());
        for srv in model.services {
            println!("{}({}): {}", srv.name,
                     srv.implements.unwrap_or(ServiceImplements{ service: "none".to_string() }).service,
                     srv.description);
        }
    }

    use glob::{glob_with, MatchOptions};

    // error_chain! {
    //     foreign_links {
    //         Glob(glob::GlobError);
    //         Pattern(glob::PatternError);
    //     }
    // }

    // ref: https://rust-lang-nursery.github.io/rust-cookbook/file/dir.html
    #[test]
    fn list_service_conf_works() -> anyhow::Result<()> {
        let cnt=std::fs::read_to_string("cc.toml")?;
        let config: CcConfig = toml::from_str(cnt.as_str())?;
        println!("ofbiz location: {}", config.ofbiz_loc);

        let options = MatchOptions {
            case_sensitive: false,
            ..Default::default()
        };

        for entry in glob_with(
            format!("{}/**/servicedef/services*.xml", config.ofbiz_loc).as_str(), options)? {
            println!("{}", entry?.display());
        }

        Ok(())
    }
}

