#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::ServiceMeta;
    use itertools::Itertools;

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
}