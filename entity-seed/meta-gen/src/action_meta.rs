use crate::cases::CaseResource;
use std::collections::HashMap;
use serde_json::Value;
use itertools::Itertools;

fn is_ofbiz_service(url: &str) -> bool {
    url.contains("ofbiz_srvs")
}

fn get_service(url: &str) -> Option<&str> {
    if is_ofbiz_service(url){
        url.split("/").last()
    }else{
        None
    }
}

fn get_content_type(res: &CaseResource) -> Option<String>{
    let cnt_type=res.headers.iter()
        .find(|h|h.name.eq_ignore_ascii_case("Content-Type"))
        .and_then(|f|Some(f.value.to_string()));
    cnt_type
}

fn is_json_body(res: &CaseResource) -> bool {
    if let Some(body)=&res.body{
        return body.mime_type=="application/json";
    }
    return false;
}

fn get_json_parameter_names(res: &CaseResource) -> crate::Result<Vec<String>>{
    if let Some(body)=&res.body{
        if body.mime_type=="application/json"{
            let paras:HashMap<String, Value>=serde_json::from_str(body.text.as_str())?;
            let r=paras.iter()
                .map(|(k,_)|k.to_string())
                .collect_vec();
            return Ok(r);
        }
    }

    Ok(Vec::default())
}

#[derive(Serialize, Deserialize, Debug)]
struct ActivityParameter{
    name: String,
    type_name: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActivityMethod{
    name: String,
    parameters: Vec<ActivityParameter>,
    #[serde(skip_deserializing)]
    input_str: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ActivityMeta{
    name: String,
    methods: Vec<ActivityMethod>,
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::ServiceMeta;
    use deles::delegators::pretty;
    use tera::{Tera, Context};

    #[test]
    fn get_service_works() -> anyhow::Result<()> {
        let url = "{{ _.ofbiz_srvs }}/updateProduct";
        if let Some(s) = get_service(url) {
            assert_eq!("updateProduct", s);
        }
        Ok(())
    }

    #[test]
    fn case_parse_works() -> anyhow::Result<()> {
        let case_raw = include_str!("fixtures/case_service.yml");
        let res: CaseResource = serde_yaml::from_str(case_raw)?;
        println!("{}", res.name);
        assert_eq!(Some("application/json".to_string()), get_content_type(&res));

        let para_names = get_json_parameter_names(&res)?;
        println!("{:?}", para_names);

        let mut srvs=ServiceMeta::load()?;

        if let Some(s) = get_service(res.url.expect("url").as_str()) {
            let mut meta = srvs.srv_params(s)?;
            if let Ok(Some(intf))=srvs.get_interface(s){
                let mut meta_intf = srvs.srv_params(intf.as_str())?;
                meta.append(&mut meta_intf);
            }
            let pnames=meta.iter().map(|p|&p.name).collect_vec();
            println!("{:?}", pnames);

            let paras=meta.iter().filter(|p|para_names.contains(&p.name))
                .map(|p|ActivityParameter{
                    name: p.name.to_string(),
                    type_name: p.param_java_type()
                })
                .collect_vec();
            let input_str=paras.iter()
                    .map(|p| format!("{} {}", p.type_name, p.name))
                    .collect_vec()
                    .join(", ");
            let method=ActivityMethod{
                name:s.to_string(),
                parameters: paras,
                input_str: input_str,
            };
            println!("{}", pretty(&method));
            let act_meta=ActivityMeta{
                name: "Hello".to_string(),
                methods: vec![method] };

            // generate code from template
            let mut tera = Tera::default();
            tera.add_raw_template("bp_act", include_str!("incls/bp_activity.j2"))?;
            tera.add_raw_template("bp_act_impl", include_str!("incls/bp_activity_impl.j2"))?;
            let mut context = Context::new();
            context.insert("act", &act_meta);

            let result = tera.render("bp_act", &context)?;
            println!("{}", result);
            let result = tera.render("bp_act_impl", &context)?;
            println!("{}", result);
        }

        Ok(())
    }
}


