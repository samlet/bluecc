use serde_xml_rs::{from_reader, from_str};
use std::str;
use std::io::prelude::*;
use crate::meta_model::EntityModel;
use serde::Deserialize;
use std::io::{Read, BufReader};

lazy_static_include_bytes! {
// lazy_static_include_str! {
    EXAMPLE_DOC => "entitydef/example-entitymodel.xml",
    ACCOUNTING_DOC => "entitydef/accounting-entitymodel.xml",
}

fn deserialize_branch_without_contiguous_check<'de, T: Deserialize<'de>>(reader: impl Read) -> T {
    let mut de = serde_xml_rs::Deserializer::new_from_reader(BufReader::new(reader));
    T::deserialize(&mut de).unwrap()
}

/// https://gist.github.com/tobz1000/dd2d91c1e8c63171a21ec2d51dc726c7
fn deserialize_branch_with_contiguous_check<'de, T: Deserialize<'de>>(reader: impl Read) -> T {
    let mut de = serde_xml_rs::Deserializer::new_from_reader(BufReader::new(reader))
        .non_contiguous_seq_elements(true);
    T::deserialize(&mut de).unwrap()
}


#[test]
fn doc_works() -> anyhow::Result<()>{
    // let _ = simple_logger::init();
    // let model:EntityModel=from_str(str::from_utf8(&EXAMPLE_DOC).unwrap())?;
    // let model:EntityModel=from_str(str::from_utf8(&ACCOUNTING_DOC).unwrap())?;
    let model:EntityModel=deserialize_branch_with_contiguous_check(&**ACCOUNTING_DOC);
    // let model:EntityModel= match from_str(str::from_utf8(&ACCOUNTING_DOC).unwrap()) {
    //     Ok(doc) => doc,
    //     Err(e) => {
    //         eprintln!("Error reading: {:#?}", e);
    //         panic!()
    //     }
    // };
    println!("{:?}, contains {} entities, {} views", model.title,
             model.entities.len(), model.views.len());
    Ok(())
}
