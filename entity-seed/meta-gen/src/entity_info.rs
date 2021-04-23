#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::ServiceMeta;
    use itertools::Itertools;
    use std::borrow::Borrow;

    #[test]
    fn rels_works() -> anyhow::Result<()> {
        let mut meta = ServiceMeta::load()?;
        let ent_name="Person";
        let ent=meta.get_entity_model(ent_name)?;

        let rels = ent.get_relation_entities();
        println!("{:?}", rels);

        let all_rels=meta.entity_reader.get_or_build_relations(ent_name)?;
        if let Some(rs)=all_rels {
            for r in rs {
                println!("{:?}", r);
            }
        }
        Ok(())
    }

    #[test]
    fn entity_type_works() -> anyhow::Result<()> {
        let mut reader=seed::meta::ModelReader::load()?;
        let names=reader.get_all_entity_names();
        let mut index=0;
        for (_i, name) in names.iter().enumerate(){
            let ent=reader.get_entity_model(name.as_str())?;
            let flds=ent.get_field_names();
            if flds.contains(&String::from("parentTypeId"))
                && flds.contains(&String::from("description")){
                println!("{}. + {} ({})", index, name, ent.pks_str());
                index+=1;
            }else if name.ends_with("Type")
                && flds.contains(&String::from("description")){
                println!("{}. - {} ({})", index, name, ent.pks_str());
                index+=1;
            }
        }
        Ok(())
    }
}