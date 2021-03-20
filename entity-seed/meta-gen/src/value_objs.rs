use seed::{EntityGenerator, ModelField};
use seed::meta::{SeedFiles, load_seed_model_z_file};
use crate::{GenericError, ServiceMeta};
use std::collections::HashMap;
use tera::Context;

fn print_seeds(name: &str) -> Result<(), GenericError>{
    load_seed_model_z_file(name, |n| {
        println!("{} ({:?})", n.tag_name().name(), n.range());
        for attr in n.attributes() {
            println!("\t{} = {}", attr.name(), attr.value());
        }
        true
    })?;
    Ok(())
}

fn gen_value_obj(meta:&mut ServiceMeta, ent_name: &str, flds: &Vec<&String>) -> Result<String, GenericError>{
    let ent=meta.entity_reader.get_entity_model(ent_name)?;
    debug!("{} => ", ent_name);
    for &f in flds{
        let fld=ent.get_field(f.as_str()).expect("fld");
        debug!("\t{}: {}", fld.field_name, fld.field_type);
    }

    let mut generator=EntityGenerator::new(vec![ent_name.to_string()]);
    generator.tera.add_raw_template("value_obj", include_str!("incls/value_obj.j2"))?;
    let mut context = Context::new();
    let ent_flds:Vec<&ModelField>= flds.iter().map(|&f|ent.get_field(f).unwrap()).collect();
    context.insert("ent", &ent);
    context.insert("flds", &ent_flds);
    let result = generator.tera.render("value_obj", &context)?;

    Ok(result)
}

#[test]
fn seed_works() -> anyhow::Result<()> {
    print_seeds("Person")?;

    Ok(())
}

#[test]
fn field_count_works() -> anyhow::Result<()> {
    let seeds=SeedFiles::load()?;
    let ent_name="Person";
    let rs=seeds.entity_seeds(ent_name)?;
    let mut stats=HashMap::new();
    for r in &rs{
        let fld_num=r.len();
        let entry=stats.entry(fld_num).or_insert(1);
        *entry+=1;
    }

    // 找到最常用的字段组合(即这个组合的频次最高)
    let max_item=stats.iter()
        .max_by(|f,s|f.1.cmp(s.1)).unwrap();
    println!("{:?} => {:?}", max_item, stats);
    let exflds=rs.iter().filter(|&r|r.len()==*max_item.0)
        .nth(0).unwrap();
    println!("{:?} => {:?}", exflds.keys(), exflds);
    let flds=exflds.keys().into_iter().collect::<Vec<&String>>();

    // 将这个字段组合生成为值对象类, 未列出的字段都收容在extra-map中
    println!("generate value_obj model => ");
    let mut srvs =ServiceMeta::load()?;
    let result=gen_value_obj(&mut srvs, ent_name, &flds)?;
    println!("{}", result);

    Ok(())
}

