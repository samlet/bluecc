use seed::{SeedProcessor, SeedTypes, GenericError, StringStore, SerialKey,
           get_entity_model};
use seed::meta::SeedFiles;
use chrono::NaiveDateTime;
use roxmltree::Node;
use std::io::prelude::*;
use serde::Deserialize;

trait SeedGen{
    fn gen_json(&self, xml_str: &str) -> Result<Vec<SeedTypes>, GenericError>;
}

impl SeedGen for SeedProcessor {
    fn gen_json(&self, xml_str: &str) -> Result<Vec<SeedTypes>, GenericError> {
        use chrono::format::strftime::StrftimeItems;

        let parse_dt = NaiveDateTime::parse_from_str;
        let fmt = StrftimeItems::new("%Y-%m-%dT%H:%M:%S");
        let mut store = StringStore::load().unwrap();

        let doc = roxmltree::Document::parse(xml_str).unwrap();
        let root = doc.root_element();
        let nodes = root.children()
            .filter(|n| n.is_element()).collect::<Vec<Node<'_, '_>>>();
        // let model=get_entity_model("security");

        let mut result_set = Vec::new();
        for node in nodes {
            let node_name = node.tag_name().name();
            // let mod_ent=get_entity_by_name(node_name).expect("entity");
            let mod_ent = get_entity_model(node_name)?;
            let flds: Vec<String> = node.attributes().iter().map(|f| {
                let fld_name= self.revs.get_field_rev(node_name, f.name());

                let mut fld_val: String = f.value().to_string();
                let mod_fld = mod_ent.get_field(fld_name.as_str()).expect("field");
                if mod_fld.is_dt_type() {
                    fld_val = parse_dt(f.value(), "%Y-%m-%d %H:%M:%S%.f").unwrap()
                        .format_with_items(fmt.clone()).to_string();
                } else if mod_fld.is_id_type() {
                    let mut rel_ent = node_name.to_string();
                    match mod_ent.get_relation_entity(fld_name.as_str()) {
                        Some(r) => rel_ent = r,
                        _ => ()
                    }
                    let val = store.id(SerialKey::new(rel_ent.as_str(), f.name(), f.value()));
                    fld_val = val.to_string();
                } else if mod_fld.field_type == "indicator" {
                    fld_val = match fld_val.as_str() {
                        "Y" => "true".to_string(),
                        "N" => "false".to_string(),
                        _ => "false".to_string()
                    };
                }

                format!(" {}=\"{}\"", fld_name, fld_val)
            }).collect();

            let node_str = format!("<{} {}/>", node.tag_name().name(), flds.join(" "));
            let data: SeedTypes = serde_xml_rs::from_str(node_str.as_str()).unwrap();
            debug!("{} ->\n  {:?}", node_str, data);
            result_set.push(data);
        }

        store.store()?;
        Ok(result_set)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::read_to_string;

    #[test]
    fn it_works() -> anyhow::Result<()> {
        Ok(())
    }

    #[test]
    fn process_gen_json() -> anyhow::Result<()> {
        std::env::set_var("RUST_LOG", "info,entity_seed=debug");
        env_logger::init();
        debug!("process seed xml ...");

        let procs=SeedProcessor::default();

        let cnt=read_to_string("../data/security/SecurityGroupDemoData.xml")?;
        // let cnt=read_to_string("data/security/SecurityPermissionSeedData.xml")?;
        // let cnt=read_to_string("../data/common/CountryCodeData.xml")?;
        let rs=procs.gen_json(cnt.as_str())?;
        println!("total {}", rs.len());
        for rec in rs {
            match rec {
                SeedTypes::SecurityGroup(e) => {
                    println!("security-group id {:?}\n{}", e.group_id, serde_json::to_string_pretty(&e)?);
                }
                SeedTypes::UserLogin(e) => {
                    println!("user-login id {:?}\n{}", e.user_login_id,
                             serde_json::to_string_pretty(&e)?);
                    // store it to db
                }
                SeedTypes::CountryCapital(ref e) => {
                    println!("{:?}; {}",  e.country_capital_name,
                             serde_json::to_string_pretty(&e)?);
                }
                SeedTypes::CountryCode(ref e) => {
                    println!("{:?}; {}", e.country_code_id,
                             serde_json::to_string_pretty(&e)?);
                }
                _ => ()
            }
        }
        Ok(())
    }

    #[test]
    fn seed_files_works() -> anyhow::Result<()> {
        let seeds=SeedFiles::load()?;
        for f in seeds.data_files.files{
            println!("{} - {}", f.uri, f.path);
        }
        Ok(())
    }
}

