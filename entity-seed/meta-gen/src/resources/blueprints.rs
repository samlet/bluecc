#[cfg(test)]
mod lib_tests {
    use super::*;
    use redis::Commands;
    use serde_json::json;
    use crate::xml_seed::SeedValue;
    use trees::Tree;
    use std::convert::TryFrom;
    use crate::pprint_tree;
    use itertools::Itertools;
    use bstr::ByteSlice;

    #[test]
    fn tree_data_works() -> anyhow::Result<()> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;

        let key = "tree_obj_key";
        let tree_val="0 (1 2)";
        redis::cmd("tree.init").arg(key)
            .arg(tree_val)
            .query(&mut con)?;
        let tree_val="3 ( 4 5)";
        redis::cmd("tree.set_subtree").arg(key)
            .arg("2")
            .arg(tree_val)
            .query(&mut con)?;

        let val: String = redis::cmd("tree.get")
            .arg(key).query(&mut con)?;
        println!("New value: {}", val);
        assert_eq!("0( 1 2( 3( 4 5 ) ) )", val);

        Ok(())
    }

    #[test]
    fn tree_string_data_works() -> anyhow::Result<()> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;

        let key = "string_obj_key";
        let tree_string="USA (Legislature (House (Pelosi) Senate (Harris))\
            ExecutiveJudiciary (WhiteHouse (Biden))\
            Judiciary (SupremeCourt (Roberts)))";
        redis::cmd("tree.init").arg(key)
            .arg(tree_string)
            .query(&mut con)?;
        let tree_val="3 ( order1:4 5)";
        redis::cmd("tree.set_subtree").arg(key)
            .arg("Roberts")
            .arg(tree_val)
            .query(&mut con)?;

        let val: String = redis::cmd("tree.get")
            .arg(key).query(&mut con)?;
        println!("New value: {}", val);

        let piled=Tree::try_from(String::from(val))?;
        println!("{}", piled.to_string());
        println!("{:?}", piled.root().locate_first_by_data(&"Judiciary".to_string()).unwrap()
            .descendants());

        pprint_tree(&piled, false);

        // sub tree
        let val: String = redis::cmd("tree.get_subtree")
            .arg(key).arg("Judiciary").query(&mut con)?;
        let piled=Tree::try_from(String::from(val))?;
        pprint_tree(&piled, false);

        Ok(())
    }

    #[test]
    fn pks_works() -> anyhow::Result<()> {
        use base64::{encode, decode, DecodeError};
        let seed:SeedValue=serde_json::from_value(json!({
            "entity": "ExampleItem",
            "values": {
              "amount": 10,
              "exampleId": "EX01",
              "description": "EX1-001",
              "exampleItemSeqId": "00001"
            }
          }))?;
        let ent=seed::get_entity_model(seed.entity.as_str())?;
        let pk_vals=ent.pks().iter().map(|f|seed.values.get(f).as_ref().unwrap().as_str().unwrap())
            .join(":");
        let pk_vals_encode=encode(pk_vals.as_str());
        println!("pks: {}, {}", &pk_vals, pk_vals_encode);
        println!("gid: {}, {}", seed.get_global_id()?, seed.get_encoded_global_id()?);

        Ok(())
    }

    fn elems_gid(seeds: &Vec<SeedValue>, ent_name: &str) -> Vec<String>{
        let elems:Vec<String>=seeds.iter().filter(|e|e.entity==ent_name)
            .map(|e|e.get_global_id().expect("get encoded gid"))
            .collect();
        elems
    }

    #[test]
    fn load_seeds_works() -> anyhow::Result<()> {
        use base64::{encode};

        let file=".store/ExampleDemoData.json";
        let seeds:Vec<SeedValue>=serde_json::from_reader(std::fs::File::open(file)?)?;
        println!("total {}", seeds.len());
        let ent_name="Example";
        let elems=elems_gid(&seeds, ent_name);
        let init_tree=format!("{} ({})", ent_name, elems.join(" "));
        println!("{}", init_tree);
        let mut piled=Tree::try_from(String::from(init_tree))?;

        let child_ent="ExampleItem";
        fn match_parent_id(seed: &SeedValue, parent_ent:&str, fld:&str, parent_id:&str) -> bool {
            let pid=format!("{}:{}", parent_ent,
                    seed.values.get(fld).as_ref().unwrap().as_str().unwrap());
            // let result=encode(pid);
            // println!("{} == {}", result, parent_id);
            // result==parent_id
            pid==parent_id
        }

        for elem in elems{
            let children:Vec<String>=seeds.iter()
                .filter(|e|e.entity==child_ent &&
                    match_parent_id(e, "Example", "exampleId",elem.as_str()))
                .map(|e|e.get_global_id().expect("get encoded gid"))
                .collect();
            // println!("children size: {}", children.len());

            if !children.is_empty() {
                // let subtree_str=format!("{} ({})", elem, children.join(" "));
                let subtree_str = format!("{} ({})", "elements", children.join(" "));
                let subtree = Tree::try_from(subtree_str)?;
                if let Some(mut node) = piled.root_mut().locate_first_mut_by_data(&elem) {
                    node.push_back(subtree);
                }
            }
        }

        pprint_tree(&piled, false);

        Ok(())
    }

    #[test]
    fn decode_works() -> anyhow::Result<()> {
        use base64::{decode, DecodeError};
        let elem="RXhhbXBsZUl0ZW06RVgwMjowMDAwMQ==";
        let convert=true;
        let data_str=if convert
            {decode(elem.to_string()).expect("decode").to_str_lossy().to_string()}
            else {elem.to_string()};
        println!("{}", data_str);
        Ok(())
    }
}

