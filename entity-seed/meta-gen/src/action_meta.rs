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

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::ServiceMeta;

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
                .map(|p|(p.name.to_string(), p.type_name.to_string()))
                .collect_vec();
            println!("{:?}", paras);
        }
        Ok(())
    }
}


