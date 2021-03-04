use serde_xml_rs::{from_reader, from_str};
use std::str;
use itertools::Itertools;
use phf::{phf_map};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct Entity{
    #[serde(rename = "entity-name", default)]
    pub entity_name: String,
    #[serde(rename = "field", default)]
    pub fields: Vec<ModelField>,
    #[serde(rename = "prim-key", default)]
    pub primary_keys: Vec<PrimKey>,
    #[serde(rename = "relation", default)]
    pub relations: Vec<ModelRelation>,
    #[serde(skip_deserializing)]
    pub multiple_keys: bool
}

impl Entity{
    pub fn pks_str(&self) -> String{
        use inflector::cases::snakecase::to_snake_case;
        let pks:Vec<String>=self.primary_keys.iter().map(|x|
            to_snake_case(&x.field_name.as_str())).collect();
        pks.iter().join(", ").to_string()
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PrimKey{
    #[serde(rename = "field", default)]
    pub field_name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelField{
    #[serde(rename = "name", default)]
    pub field_name: String,
    #[serde(rename = "type", default)]
    pub field_type: String,
    #[serde(skip_deserializing)]
    pub is_primary: bool
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ModelRelation{
    // one of: one, many, one-nofk
    #[serde(rename = "type", default)]
    pub rel_type: String,
    #[serde(rename = "fk-name", default)]
    pub fk_name: String,
    #[serde(rename = "title", default)]
    pub title: String,
    #[serde(rename = "rel-entity-name", default)]
    pub rel_entity_name: String,
    #[serde(rename = "key-map", default)]
    pub keymaps: Vec<KeyMap>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeyMap{
    #[serde(rename = "field-name", default)]
    pub field_name: String,
    #[serde(rename = "rel-field-name", default)]
    pub rel_field_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EntityModel{
    pub title: String,
    pub description: String,
    pub version: String,
    #[serde(rename = "default-resource-name", default)]
    pub default_resource_name: String,
    #[serde(rename = "entity", default)]
    pub entities: Vec<Entity>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FieldTypeDef{
    #[serde(rename = "type", default)]
    pub field_type: String,
    #[serde(rename = "sql-type", default)]
    pub sql_type: String,
    #[serde(rename = "java-type", default)]
    pub java_type: String,
    #[serde(rename = "query-type", default)]
    pub query_type: String,
    #[serde(rename = "insert-type", default)]
    pub insert_type: String
}

#[derive(Debug, Deserialize)]
pub struct FieldTypes{
    #[serde(rename = "field-type-def", default)]
    pub field_types: Vec<FieldTypeDef>
}

impl FieldTypes{
    fn get_field(&self, field_type:&str) -> &FieldTypeDef{
        self.field_types.iter()
            .find(|x| x.field_type==field_type).unwrap()
    }
    pub fn sql_type(&self, field_type:&str) -> String{
        self.get_field(field_type).sql_type.clone()
    }
    pub fn query_type(&self, field_type:&str) -> String{
        let fld=self.get_field(field_type);
        if fld.query_type.is_empty(){
            if fld.java_type=="String"{
                "String".to_string()
            }else{
                format!("**UNK({})**", field_type)
            }
        }else{
            fld.query_type.clone()
        }
    }
    pub fn insert_type(&self, field_type:&str) -> String{
        let fld=self.get_field(field_type);
        if fld.insert_type.is_empty(){
            if !fld.query_type.is_empty(){
                fld.query_type.clone()
            }
            else if fld.java_type=="String"{
                "&'a str".to_string()
            }else{
                format!("**UNK({})**", field_type)
            }
        }else{
            fld.insert_type.clone()
        }
    }
}

