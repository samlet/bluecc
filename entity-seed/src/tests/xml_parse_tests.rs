use std::str;

/// ref: https://github.com/RazrFalcon/roxmltree/blob/master/tests/dom-api.rs
#[test]
fn seed_works(){
    let xml_str=str::from_utf8(include_bytes!("ExampleDemoData.xml")).unwrap();
    let doc = roxmltree::Document::parse(xml_str).unwrap();
    let elem = doc.descendants().find(|n|
        n.attribute("statusTypeId") == Some("EXAMPLE_STATUS")).unwrap();
    assert!(elem.has_tag_name("StatusType"));

    let root_elem=doc.root().first_element_child();
    let iter=root_elem.unwrap().children();
    for n in iter{
        println!("{}", n.tag_name().name());
    }
}

#[test]
fn reader_works() {
    // https://simplabs.com/blog/2020/12/31/xml-and-rust/
    // https://stackoverflow.com/questions/19076719/how-do-i-convert-a-vector-of-bytes-u8-to-a-string
    let xml_str=include_bytes!("simple.xml");
    let parser = xml::reader::EventReader::from_str(str::from_utf8(xml_str).unwrap());
    for event in parser {
        println!("{:?}", event.unwrap());
    }
}


