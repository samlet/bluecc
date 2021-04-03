use std::collections::{HashSet, HashMap};
use std::collections::hash_map::RandomState;
use chrono::{NaiveDateTime, NaiveDate};
use roxmltree::Node;
use crate::DynamicValue;
use deles::delegators::pretty;
use std::path::PathBuf;

fn get_seed_entities(cnt:&str) -> crate::Result<HashSet<String>> {
    let doc = roxmltree::Document::parse(cnt)?;

    let root = doc.root_element();
    let excepts=vec!["create", "create-replace", "create-update", "delete"];
    let ents=root.children().into_iter()
        .filter(|e| e.is_element() && !excepts.contains(&e.tag_name().name()))
        .map(|e| e.tag_name().name().to_string())
        .collect::<HashSet<String>>();
    Ok(ents)
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct SeedValue {
    pub entity: String,
    pub values: HashMap<String, serde_json::Value>,
}

pub fn process_seed(xml_str: &str) -> crate::Result<Vec<SeedValue>> {
    use chrono::format::strftime::StrftimeItems;
    use serde_json::{json,Value};
    use crate::META_CONF;

    let trans_indicator=META_CONF.xml_seed.translate_indicator;
    let trans_dt=META_CONF.xml_seed.translate_date_time;

    let parse_dt = NaiveDateTime::parse_from_str;
    let fmt = StrftimeItems::new("%Y-%m-%dT%H:%M:%S");

    let doc = roxmltree::Document::parse(xml_str).unwrap();
    let root = doc.root_element();
    let nodes = root.children()
        .filter(|n| n.is_element()).collect::<Vec<Node<'_, '_>>>();

    let mut result_set = Vec::new();
    let excepts=vec!["create", "create-replace", "create-update", "delete"];
    for node in nodes {
        let node_name = node.tag_name().name();
        if excepts.contains(&node_name){
            continue; // skip it
        }
        // let mod_ent=get_entity_by_name(node_name).expect("entity");
        let mod_ent = seed::get_entity_model(node_name)?;
        let flds: Vec<(String, serde_json::Value)> = node.attributes().iter().map(|f| {
            let fld_name= f.name();

            let mut fld_val: serde_json::Value = Value::from(f.value().to_string());
            // let mod_fld = mod_ent.get_field(fld_name)
            //     .expect(format!("no field {} in entity {}", fld_name, node_name).as_str());
            let opt_fld=mod_ent.get_field(fld_name);
            if let Some(mod_fld)=opt_fld {
                if mod_fld.is_dt_type() {
                    if trans_dt {
                        if let Ok(dt) = parse_dt(f.value(), "%Y-%m-%d %H:%M:%S%.f") {
                            fld_val = dt.format_with_items(fmt.clone()).to_string().into();
                        } else if let Ok(_dt) = NaiveDate::parse_from_str(f.value(), "%Y-%m-%d") {
                            // just a check, leave it as string value
                        } else {
                            // leave it as string value
                            warn!("cannot parse date-time value {}", f.value());
                        }
                    }
                } else if mod_fld.is_num_type() {
                    let fval= if f.value().contains(".") {f.value()}
                        else {f.value().trim_start_matches('0')};
                    if fval.is_empty() {
                        fld_val=json!(0);
                    }else {
                        fld_val = Value::Number(fval.parse()
                            .expect(format!("invalid number parse: {}, context: {:?}",
                                            f.value(), node).as_str()));
                    }
                } else if mod_fld.field_type == "indicator" {
                    if trans_indicator {
                        fld_val = match f.value() {
                            "Y" => Value::Bool(true),
                            "N" => Value::Bool(false),
                            _ => Value::Bool(false)
                        };
                    }
                }
            }else{
                warn!("no field {} in entity {}", fld_name, node_name);
            }

            (fld_name.to_string(), fld_val)
        }).collect();

        let mut values=SeedValue{ entity: node_name.to_string(), values: Default::default() };
        for (k,v) in flds{
            values.values.insert(k,v);
        }
        result_set.push(values);
    }

    Ok(result_set)
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn xml_seed_works() -> crate::Result<()> {
        use std::fs;
        let cnt = fs::read_to_string("test_files/ExampleDemoData.xml")?;
        let ents = get_seed_entities(cnt.as_str())?;
        let r = serde_json::to_string_pretty(&ents)?;
        println!("{}", r);
        Ok(())
    }

    #[test]
    fn convert_works() -> crate::Result<()> {
        use std::fs;
        let path = PathBuf::from("test_files/ExampleDemoData.xml");
        let cnt = fs::read_to_string(path.as_path())?;
        let rs = process_seed(cnt.as_str())?;
        println!("{}", pretty(&rs));
        let output: PathBuf = PathBuf::from(".store").join(path.file_name().unwrap()).with_extension("json");
        println!("{}", output.display());
        fs::write(output.as_path(), pretty(&rs))?;

        Ok(())
    }
}

