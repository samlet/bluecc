use seed::meta::{ServiceModelReader, ServiceModel,
                 ModelReader,
                 ModelService, ServiceImplements};
use seed::{new_snowflake_id, GenericError, Entity};

fn ex_service_models() -> ServiceModel{
    let bytes:&[u8]=include_bytes!("fixtures/services.xml");
    serde_xml_rs::from_reader(bytes).expect("ex srvs")
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


#[test]
fn model_manager_works() -> anyhow::Result<()> {
    let sr = ServiceModelReader::new()?;
    let create_srvs:Vec<String>=sr.get_all_service_names().iter()
        .filter(|&s| s.starts_with("create"))
        .map(|s| s.clone())
        .collect();
    println!("total {}, top 5:", create_srvs.len());
    for srv in &create_srvs[0..5]{
        println!("{}", srv);
    }
    Ok(())
}

struct ServiceMeta{
    pub service_reader: ServiceModelReader,
    pub entity_reader: ModelReader,
}

impl ServiceMeta{
    pub fn load() -> Result<Self, GenericError> {
        Ok(ServiceMeta {
            service_reader: ServiceModelReader::new()?,
            entity_reader: ModelReader::load()?,
        })
    }

    pub fn srv_ent(&mut self, srv_name: &str) -> Result<Entity, GenericError> {
        let srv=self.service_reader.get_service_model(srv_name)?;
        if !srv.default_entity_name.is_empty() {
            self.entity_reader.get_entity_model(srv.default_entity_name.as_str())
        } else {
            Err(GenericError::NotFound {
                item_name: srv.default_entity_name.clone(),
                info: format!("cannot find entity {}", srv.default_entity_name)
            })
        }
    }
}

#[test]
fn service_meta_works() -> anyhow::Result<()> {
    let mut srvs =ServiceMeta::load()?;
    let srv = srvs.service_reader.get_service_model("createSecurityGroup")?.to_owned();
    let srv_json = serde_json::to_string_pretty(&srv)?;
    println!("srv {}", srv_json);
    let ent = srvs.srv_ent(srv.name.as_str())?;
    println!("srv ent {}: {}", ent.entity_name, ent.title);

    Ok(())
}

