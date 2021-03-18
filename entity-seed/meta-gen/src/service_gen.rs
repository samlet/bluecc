use seed::meta::{ServiceModelReader, ServiceModel,
                 ModelReader, ServiceAutoAttributes,
                 ModelService, ServiceImplements};
use seed::{new_snowflake_id, GenericError, Entity, ModelField, FIELD_MAPPINGS};
use std::collections::HashSet;
use inflector::Inflector;

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

fn extract_auto_attrs<'a>(ent: &'a Entity, filter: &ServiceAutoAttributes) -> Vec<&'a ModelField>{
    let include_pk= filter.include=="pk" || filter.include=="all";
    let include_non_pk= filter.include=="nonpk" || filter.include=="all";
    let excludes:Vec<String>=filter.excludes.iter()
        .map(|ex|ex.name.clone()).collect();
    ent.fields.iter()
        .filter(|&f| !excludes.contains( &f.field_name))
        .filter(|&f| {
            (f.is_primary && include_pk) || (!f.is_primary && include_non_pk)
        })
        .collect()
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
enum ParamMode{
    In, Out, InOut
}
#[derive(Debug, Serialize, Deserialize, Clone)]
struct ModelParam{
    pub name: String,
    #[serde(default)]
    pub description: Option<String>,
    pub type_name: String,
    pub mode: ParamMode,
    pub form_label: Option<String>,
    pub entity_name: Option<String>,
    pub field_name: Option<String>,
    #[serde(default)]
    pub optional: bool,
    #[serde(default)]
    pub internal: bool,
    pub default_value: Option<String>,
}

impl From<&str> for ParamMode {
    fn from(item: &str) -> Self {
        match item {
            "IN" => ParamMode::In,
            "OUT" => ParamMode::Out,
            "INOUT" => ParamMode::InOut,
            _ => ParamMode::InOut
        }
    }
}

#[test]
fn service_meta_works() -> anyhow::Result<()> {
    let mut srvs =ServiceMeta::load()?;
    // createSecurityGroup, createExampleType, updateExample, testCreateExampleService
    // let srv = srvs.service_reader.get_service_model("testCreateExampleService")?.to_owned();
    let srv = srvs.service_reader.get_service_model("updateExample")?.to_owned();
    let srv_json = serde_json::to_string_pretty(&srv)?;
    println!("srv {}", srv_json);

    let mut all_flds = Vec::new();
    let mut params=Vec::new();

    // process entity-auto-attrs
    if !srv.default_entity_name.is_empty() {
        println!("srv ent {:?}", srv.default_entity_name);

        let ent = srvs.srv_ent(srv.name.as_str())?;
        println!("srv ent {}: {}", ent.entity_name, ent.title);

        // let mut in_set=HashSet::new();
        // let mut out_set=HashSet::new();
        for auto_attr in &srv.auto_attributes {
            let flds = extract_auto_attrs(&ent, auto_attr);

            let mode: ParamMode = auto_attr.mode.as_str().into();
            /*
                match auto_attr.mode.as_str() {
                    // "IN" => flds.iter().map(|f| f.field_name.to_owned())
                    //     .for_each(|f| { in_set.insert(f); }),
                    "IN" => ParamMode::In,
                    "OUT" => ParamMode::Out,
                    "INOUT" => ParamMode::InOut,
                    _ => ParamMode::InOut
                };
            */
            all_flds.push((mode, auto_attr.optional, flds));
        }

        println!("all fields ->");
        for (mode, _, flds) in &all_flds {
            println!("==> {:?}", mode);
            for f in flds {
                println!("\t {}: {}", f.field_name, f.field_type);
            }
        }

        for (mode, opt, flds) in &all_flds {
            let mut fld_opt = *opt;
            let mut fld_mode = mode.to_owned();
            let mut fld_def_val = "";
            for f in flds {
                // do attributes override
                for ov in &srv.overrides {
                    if ov.name == f.field_name {
                        if let Some(opt_val) = ov.optional {
                            fld_opt = opt_val;
                        }
                        if let Some(mode_val) = &ov.mode {
                            fld_mode = mode_val.as_str().into();
                        }
                        if let Some(def_val) = &ov.default_value {
                            fld_def_val = def_val;
                        }
                    }
                }

                // convert to service-parameters
                params.push(
                    ModelParam {
                        name: f.field_name.to_string(),
                        description: None,
                        type_name: f.field_type.to_string(),
                        mode: fld_mode,
                        form_label: None,
                        entity_name: if !srv.default_entity_name.is_empty() { Some(srv.default_entity_name.to_owned()) } else { None },
                        field_name: Some(f.field_name.to_owned()),
                        optional: fld_opt,
                        internal: false,
                        default_value: if fld_def_val.is_empty() { None } else { Some(fld_def_val.to_string()) },
                    });
            }
        }
    }

    // process services attrs
    let attrs:Vec<ModelParam>= srv.attributes.iter()
        .map(|att| ModelParam{
            name: att.name.to_string(),
            description: None,
            type_name: att.data_type.to_string(),
            mode: att.mode.as_str().into(),
            form_label: None,
            entity_name: None,
            field_name: None,
            optional: att.optional,
            internal: false,
            default_value: None
        })
        .collect();

    params.extend(attrs);

    println!("all params ->");
    for f in params{
        let mut qtype=f.type_name.to_owned();
        if !f.type_name.is_title_case(){
            qtype= FIELD_MAPPINGS.query_type(f.type_name.as_str());
        }
        println!("\t {}: {}/{} ({:?},{})", f.name,
                 f.type_name, qtype, f.mode,
                 if f.optional {"optional"} else {"required"});
    }

    // ... impl-service: 直接返回接口类的参数定义, 并在其上做overrides操作
    if let Some(imp) = srv.implements{
        println!("impl -> {:?}", imp);
        let intf=srvs.service_reader.get_service_model(imp.service.as_str())?;
        let intf_json=serde_json::to_string_pretty(intf)?;
        println!("{}", intf_json);
    }

    Ok(())
}
