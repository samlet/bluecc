use std::str;
use serde_xml_rs::from_str;
use decimal::prelude::*;

use crate::meta_model::*;

lazy_static_include_bytes! {
    EXAMPLE_DOC => "src/tests/ExampleForms.xml",
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
struct Item {
    name: String,
    source: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelForms{
    #[serde(rename = "form", default)]
    pub forms: Vec<ModelForm>,
    #[serde(rename = "grid", default)]
    pub grids: Vec<ModelGrid>,
}

#[derive(Debug, Serialize, Deserialize)]
struct ModelForm{
    pub name: String,
    #[serde(rename = "type", default)]
    pub form_type: String,
    #[serde(default)]
    pub target: String,
}
#[derive(Debug, Serialize, Deserialize)]
struct ModelGrid{
    #[serde(rename = "list-name", default)]
    pub list_name: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::load_xml;

    #[test]
    fn form_parse_works() {
        let model:ModelForms=load_xml(&**EXAMPLE_DOC);
        println!("total forms {}, grids {}", model.forms.len(), model.grids.len());
    }
}
