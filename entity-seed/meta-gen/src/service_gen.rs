use seed::meta::{ServiceModelReader, ServiceModel,
                 ModelReader, ServiceAutoAttributes,
                 ModelService, ServiceImplements};
use seed::{new_snowflake_id, load_xml, GenericError, Entity, ModelField, FIELD_MAPPINGS};
use std::collections::{HashSet, HashMap};
use inflector::Inflector;
use std::io::Write;

pub struct ServiceMeta{
    pub service_reader: ServiceModelReader,
    pub entity_reader: ModelReader,
}

impl ServiceMeta{
    pub fn load() -> Result<Self, GenericError> {
        Ok(ServiceMeta {
            service_reader: ServiceModelReader::new()?,
            entity_reader: ModelReader::load()?,
            // entity_reader: seed::APP_CONTEXT.reader,
        })
    }

    pub fn srv(&mut self, srv_name: &str) -> Result<&ModelService, GenericError> {
        self.service_reader.get_service_model(srv_name)
    }

    pub fn srv_and_ent(&mut self, srv_name: &str) -> Result<(ModelService, HashMap<String, Entity>), GenericError> {
        let mut ents=HashMap::new();
        let srv=self.service_reader.get_service_model(srv_name)?;
        if !srv.get_entity_name().is_empty() {
            let ent=self.entity_reader.get_entity_model(srv.get_entity_name().as_str())?;
            ents.insert(ent.entity_name.to_owned(), ent.clone());
        }

        Ok((srv.clone(), ents))
    }

    pub fn get_entity_model(&mut self, ent_name: &str) -> Result<Entity, GenericError> {
        self.entity_reader.get_entity_model(ent_name)
    }

    pub fn srv_ent(&mut self, srv_name: &str) -> Result<Entity, GenericError> {
        let srv=self.service_reader.get_service_model(srv_name)?;
        let default_ent=ServiceMeta::get_entity_name(srv);

        if !default_ent.is_empty() {
            self.entity_reader.get_entity_model(default_ent.as_str())
        } else {
            Err(GenericError::NotFound {
                item_name: default_ent.clone(),
                info: format!("cannot find entity {}", default_ent)
            })
        }
    }

    pub fn srv_params(&mut self, srv_name: &str) -> Result<Vec<ModelParam>, GenericError> {
        debug!("get srv {} meta ..", srv_name);
        let srv = self.service_reader.get_service_model(srv_name)?.to_owned();
        let mut ents=HashMap::new();
        let ent_name=srv.get_entity_name();
        if !ent_name.is_empty() {
            debug!("srv ent {:?}", ent_name);
            let ent = self.srv_ent(srv.name.as_str())?;
            ents.insert(ent_name, ent);
        }
        ServiceMeta::srv_model_params(&srv, &ents)
    }

    pub fn srv_input_params(&mut self, srv_name: &str) -> Result<Vec<ModelParam>, GenericError> {
        let params=self.srv_params(srv_name)?;
        let r=params.iter().filter(|p|p.mode==ParamMode::In || p.mode==ParamMode::InOut)
            .cloned()
            .collect();
        Ok(r)
    }

    pub fn srv_output_params(&mut self, srv_name: &str) -> Result<Vec<ModelParam>, GenericError> {
        let params=self.srv_params(srv_name)?;
        let r=params.iter().filter(|p|p.mode==ParamMode::Out || p.mode==ParamMode::InOut)
            .cloned()
            .collect();
        Ok(r)
    }

    fn get_entity_name(srv: &ModelService) -> String{
        srv.get_entity_name()
    }

    pub fn srv_model_params(srv: &ModelService, ents: &HashMap<String, Entity>) -> Result<Vec<ModelParam>, GenericError> {
        let srv_json = serde_json::to_string_pretty(&srv)?;
        debug!("srv {}", srv_json);

        let mut all_flds = Vec::new();
        let mut params = Vec::new();

        let default_ent=ServiceMeta::get_entity_name(srv);

        // process entity-auto-attrs
        if !default_ent.is_empty() {
            debug!("srv ent {:?}", default_ent);

            // let ent = self.srv_ent(srv.name.as_str())?;
            let ent= ents.get(default_ent.as_str()).expect("ent");
            debug!("srv ent {}: {}", ent.entity_name, ent.title);

            for auto_attr in &srv.auto_attributes {
                let flds = extract_auto_attrs(ent, auto_attr);
                let mode: ParamMode = auto_attr.mode.as_str().into();
                all_flds.push((mode, auto_attr.optional, flds));
            }

            debug!("all fields ->");
            for (mode, opt, flds) in &all_flds {
                debug!("==> {:?}", mode);
                for f in flds {
                    debug!("\t {}: {} opt({})", f.field_name, f.field_type, *opt);
                }
            }

            for (mode, opt, flds) in &all_flds {
                let mut fld_opt = *opt;
                let mut fld_mode = mode.to_owned();
                let mut fld_def_val = "";
                for f in flds {
                    // convert to service-parameters
                    params.push(
                        ModelParam {
                            name: f.field_name.to_string(),
                            description: None,
                            type_name: f.field_type.to_string(),
                            mode: fld_mode,
                            form_label: None,
                            entity_name: if !default_ent.is_empty() { Some(default_ent.to_owned()) } else { None },
                            field_name: Some(f.field_name.to_owned()),
                            optional: fld_opt,
                            internal: false,
                            default_value: if fld_def_val.is_empty() { None } else { Some(fld_def_val.to_string()) },
                            overload: false
                        });
                }
            }
        }

        for mut param in params.iter_mut() {
            // do attributes override
            for ov in &srv.overrides {
                if ov.name == param.name {
                    if let Some(opt_val) = ov.optional {
                        param.optional=opt_val;
                    }
                    if let Some(mode_val) = &ov.mode {
                        param.mode = mode_val.as_str().into();
                    }
                    if let Some(def_val) = &ov.default_value {
                        param.default_value = Some(def_val.to_owned());
                    }
                    param.overload=true;
                }
            }
        }

        // process services attrs
        let attrs: Vec<ModelParam> = srv.attributes.iter()
            .map(|att| ModelParam {
                name: att.name.to_string(),
                description: None,
                type_name: att.data_type.to_string(),
                mode: att.mode.as_str().into(),
                form_label: None,
                entity_name: None,
                field_name: None,
                optional: att.optional,
                internal: false,
                default_value: None,
                overload: false
            })
            .collect();

        let all_params= params.into_iter()
            .chain(attrs.into_iter()).collect();
        // params.extend(attrs);
        Ok(all_params)
    }

    pub fn get_related_srvs(&mut self, ent: &str) -> Result<Vec<String>, GenericError> {
        let all_names = self.service_reader.get_all_service_names();
        let mut result=Vec::new();
        for srv_name in &all_names {
            let model = self.service_reader.get_service_model(srv_name.as_str())?.to_owned();
            if model.get_entity_name()==ent{
                result.push(model.name.to_owned());
            }
        }
        Ok(result)
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

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum ParamMode{
    In, Out, InOut
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelParam{
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
    #[serde(default)]
    pub overload: bool,
}

impl ModelParam{
    pub fn param_type(&self) -> String{
        let mut qtype = self.type_name.to_owned();
        if !self.type_name.is_pascal_case() {
            qtype = FIELD_MAPPINGS.orig_type(self.type_name.as_str());
        }
        qtype
    }

    pub fn param_value_type(&self, spec:&str) -> String {
        let raw_type= &self.type_name;
        let mut val:String = FIELD_MAPPINGS.orig_type(raw_type);
        if val.starts_with("Option") {
            val=val.chars().skip_while(|&c|c!='<')
                .skip(1)
                .take_while(|&c|c!='>').collect();
        }

        if val=="String" && spec.is_empty() { "&'a str".to_string() } else {val}
    }
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

fn write_service_params_check_result() -> anyhow::Result<()> {
    use std::io::Write;
    use std::fs::File;

    let mut srvs = ServiceMeta::load()?;
    // notice: take about 4s
    let mut total=0;
    let mut skip_srvs=Vec::new();
    let output=".store/spec-srvs.txt";
    let mut buffer = File::create(output)?;
    let all_names=srvs.service_reader.get_all_service_names();
    for srv_name in &all_names {
        let model=srvs.service_reader.get_service_model(srv_name.as_str())?.to_owned();
        let params = srvs.srv_params(srv_name.as_str()).unwrap_or(Default::default());
        if params.is_empty(){
            if !srv_name.starts_with("test") && !model.has_interface(){
                skip_srvs.push(srv_name.to_owned());
            }
        }
        let spec_flds = params.iter()
            .filter(|f| f.type_name == "List"
                || f.type_name == "Map"
                || f.type_name.contains(".")
            )
            .map(|f| (&f.name, &f.type_name)).collect::<Vec<(&String,&String)>>();
        if !spec_flds.is_empty() {
            writeln!(buffer, "{} spec flds: {:?}", srv_name, spec_flds)?;
            total+=1;
        }
    }
    writeln!(buffer, "total services {}, spec-srvs {}, skip {}", all_names.len(), total, skip_srvs.len())?;
    std::fs::write(".store/skip-srvs.txt", serde_json::to_string_pretty(&skip_srvs)?)?;
    Ok(())
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use serde::Deserialize;
    use std::io::{Read, BufReader};
    use std::fs::File;
    use itertools::Itertools;

    fn ex_service_models() -> ServiceModel {
        let bytes: &[u8] = include_bytes!("fixtures/services.xml");
        serde_xml_rs::from_reader(bytes).expect("ex srvs")
    }
    fn party_service_models() -> ServiceModel {
        let bytes: &[u8] = include_bytes!("fixtures/party_services.xml");
        // serde_xml_rs::from_reader(bytes).expect("ex srvs")
        let model:ServiceModel=load_xml(bytes);
        model
    }

    #[test]
    fn service_model_works() {
        // let model: ServiceModel = ex_service_models();
        let model: ServiceModel = party_service_models();
        println!("{}", model.version.unwrap());
        assert_eq!("1.0", model.version.unwrap().to_string());
        for srv in model.services {
            println!("{}({:?}): {}", srv.name,
                     srv.implements.iter().map(|i| &i.service).collect::<Vec<&String>>(),
                     srv.description);
        }
    }


    #[test]
    fn model_manager_works() -> anyhow::Result<()> {
        let sr = ServiceModelReader::new()?;
        let create_srvs: Vec<String> = sr.get_all_service_names().iter()
            .filter(|&s| s.starts_with("create"))
            .map(|s| s.clone())
            .collect();
        println!("total {}, top 5:", create_srvs.len());
        for srv in &create_srvs[0..5] {
            println!("{}", srv);
        }
        Ok(())
    }

    #[test]
    fn title_case_works() -> anyhow::Result<()> {
        assert!("BigDecimal".is_pascal_case());
        assert!(!"i32".is_pascal_case());
        assert!("Decimal".is_title_case());
        Ok(())
    }

    #[test]
    fn service_meta_works() -> anyhow::Result<()> {
        let mut srvs = ServiceMeta::load()?;
        // createSecurityGroup, createExampleType, updateExample, testCreateExampleService
        // let srv = srvs.service_reader.get_service_model("testCreateExampleService")?.to_owned();
        let srv = srvs.service_reader.get_service_model("createExample")?.to_owned();
        let srv_json = serde_json::to_string_pretty(&srv)?;
        println!("srv {}", srv_json);

        let mut all_flds = Vec::new();
        let mut params = Vec::new();

        // process entity-auto-attrs
        if !srv.get_entity_name().is_empty() {
            println!("srv ent {:?}", srv.get_entity_name());

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
            for (mode, opt, flds) in &all_flds {
                println!("==> {:?}", mode);
                for f in flds {
                    println!("\t {}: {} opt({})", f.field_name, f.field_type, *opt);
                }
            }

            for (mode, opt, flds) in &all_flds {
                let mut fld_opt = *opt;
                let mut fld_mode = mode.to_owned();
                let mut fld_def_val = "";
                for f in flds {
                    // do attributes override
                    let mut overload = false;
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
                            overload = true;
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
                            entity_name: if !srv.get_entity_name().is_empty() { Some(srv.get_entity_name().to_owned()) } else { None },
                            field_name: Some(f.field_name.to_owned()),
                            optional: fld_opt,
                            internal: false,
                            default_value: if fld_def_val.is_empty() { None } else { Some(fld_def_val.to_string()) },
                            overload: overload
                        });
                }
            }
        }

        // process services attrs
        let attrs: Vec<ModelParam> = srv.attributes.iter()
            .map(|att| ModelParam {
                name: att.name.to_string(),
                description: None,
                type_name: att.data_type.to_string(),
                mode: att.mode.as_str().into(),
                form_label: None,
                entity_name: None,
                field_name: None,
                optional: att.optional,
                internal: false,
                default_value: None,
                overload: false
            })
            .collect();

        params.extend(attrs);

        println!("all params ->");
        for f in params {
            let mut qtype = f.type_name.to_owned();
            if !f.type_name.is_title_case() {
                qtype = FIELD_MAPPINGS.query_type(f.type_name.as_str());
            }
            println!("\t {}: {}/{} ({:?},{})", f.name,
                     f.type_name, qtype, f.mode,
                     if f.optional { "optional" } else { "required" });
        }

        // ... impl-service: 直接返回接口类的参数定义, 并在其上做overrides操作
        for imp in &srv.implements {
            println!("impl -> {:?}", imp);
            let intf = srvs.service_reader.get_service_model(imp.service.as_str())?;
            let intf_json = serde_json::to_string_pretty(intf)?;
            println!("{}", intf_json);
        }

        Ok(())
    }


    #[test]
    fn service_params_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "info,entity_seed=debug,meta_gen=debug");
        env_logger::init();

        let mut srvs = ServiceMeta::load()?;
        // let params=srvs.srv_params("updateExample")?;
        let params = srvs.srv_params("createExample")?;

        println!("all params ->");
        for f in params {
            let mut qtype = f.type_name.to_owned();
            if !f.type_name.is_title_case() {
                qtype = FIELD_MAPPINGS.query_type(f.type_name.as_str());
            }
            println!("\t {}: {}/{} ({:?},{})", f.name,
                     f.type_name, qtype, f.mode,
                     if f.optional { "optional" } else { "required" });
        }

        Ok(())
    }

    #[test]
    fn service_params_check_works() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "debug,entity_seed=debug,meta_gen=debug,serde_xml_rs=info");
        env_logger::init();

        let mut srvs = ServiceMeta::load()?;
        // let params = srvs.srv_params("storeOrder")?;
        let params = srvs.srv_params("createPartyContent")?;
        println!("total params {}", params.len());
        let spec_flds=params.iter().filter(|f|f.type_name=="List" || f.type_name=="Map")
            .map(|f|&f.name).collect::<Vec<&String>>();
        println!("spec flds: {:?}", spec_flds);
        Ok(())
    }

    #[test]
    fn all_service_params_check_works() -> anyhow::Result<()> {
        write_service_params_check_result()?;
        Ok(())
    }

    #[test]
    fn rel_services_works() -> anyhow::Result<()> {
        let ent="Person";
        let mut srvs = ServiceMeta::load()?;
        let result=srvs.get_related_srvs(ent)?;
        let rels=serde_json::to_string_pretty(&result)?;
        println!("{}", rels);
        Ok(())
    }

    #[test]
    fn load_all_srvs_works() -> anyhow::Result<()> {
        let mut meta = ServiceMeta::load()?;
        let srvs=meta.service_reader.load_all_srvs()?;
        let mut buffer = File::create(".store/ent-srv-rels.txt")?;
        writeln!(buffer, "{}", srvs.len())?;

        let ents:HashSet<String>=srvs.iter().filter(|s|!s.get_entity_name().is_empty())
            .map(|&s|s.get_entity_name())
            .collect::<HashSet<String>>();
        for (i, e) in ents.iter().enumerate(){
            writeln!(buffer, "{} - {}", i, e)?;
            let rels=srvs.iter()
                .filter(|&s|s.get_entity_name()==e.as_str())
                .map(|&s|&s.name)
                .collect::<Vec<&String>>();
            for r in rels{
                writeln!(buffer, "\t - {}", r)?;
            }
        }
        Ok(())
    }

    #[test]
    fn auto_attrs_works() -> anyhow::Result<()> {
        let mut meta = ServiceMeta::load()?;
        let srv=meta.srv("createTelecomNumber")?.to_owned();
        let ent = meta.srv_ent("createTelecomNumber")?;
        println!("srv ent {}: {}", ent.entity_name, ent.title);
        for auto_attr in &srv.auto_attributes {
            let flds = extract_auto_attrs(&ent, auto_attr);
            let names=flds.iter().map(|&f|&f.field_name).collect_vec();
            println!("{:?}", names);
        }
        Ok(())
    }
}


