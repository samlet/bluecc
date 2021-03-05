use serde_xml_rs::{from_reader, from_str};
use std::str;
use crate::meta_model::EntityModel;

lazy_static_include_bytes! {
    EXAMPLE_DOC => "entitydef/example-entitymodel.xml",
    ACCOUNTING_DOC => "entitydef/accounting-entitymodel.xml",
}

#[test]
fn doc_works() -> anyhow::Result<()>{
    // let _ = simple_logger::init();
    let model:EntityModel=from_str(str::from_utf8(&EXAMPLE_DOC).unwrap())?;
    // let model:EntityModel= match from_str(str::from_utf8(&ACCOUNTING_DOC).unwrap()) {
    //     Ok(doc) => doc,
    //     Err(e) => {
    //         eprintln!("Error reading: {:#?}", e);
    //         panic!()
    //     }
    // };
    println!("{:?}", model.title);
    Ok(())
}
