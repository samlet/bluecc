use serde_xml_rs::{from_reader, from_str};
use std::str;
use itertools::Itertools;
use phf::{phf_map};
use std::collections::HashMap;
use inflector::cases::snakecase::to_snake_case;
use crate::topo::TopologicalSort;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Entity{
    #[serde(rename = "entity-name", default)]
    pub entity_name: String,
    #[serde(default)]
    pub title: String,
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

    pub fn belongs(&self) -> Vec<BelongsTo> {
        let rels = self.relations
            .iter()
            .unique_by(|x| &x.rel_entity_name)
            .filter(|x| x.single_belongs())
            .map(|x| {
            let key = &x.keymaps.get(0).unwrap();
            BelongsTo {
                field_name: to_snake_case(key.field_name.as_str()),
                model_name: x.rel_entity_name.clone(),
                rel_field_name: to_snake_case(key.get_rel_field()),
                fk_name: x.fk_name.clone()
            }
            })
            .collect::<Vec<_>>();
        rels
    }

    pub fn get_id_fields(&self) -> Vec<&String>{
        self.fields.iter().filter(|f| f.is_id_type())
            .map(|f| &f.field_name).collect()
    }

    pub fn get_field(&self, fld:&str) -> Option<&ModelField>{
        self.fields.iter().find(|f| f.field_name==fld)
    }

    pub fn get_relation_entity(&self, fld:&str) -> Option<String> {
        self.relations.iter().filter(|r|
            r.keymaps.iter().any(|key|key.field_name==fld))
            .map(|r| r.rel_entity_name.clone())
            .nth(0)
    }

    pub fn get_relation(&self, rel_name:&str) -> Option<&ModelRelation>{
        self.relations.iter().filter(|r|rel_name==r.relation_name())
            .nth(0)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PrimKey{
    #[serde(rename = "field", default)]
    pub field_name: String
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ModelField{
    #[serde(rename = "name", default)]
    pub field_name: String,
    #[serde(rename = "type", default)]
    pub field_type: String,
    #[serde(skip_deserializing)]
    pub is_primary: bool,
    #[serde(skip_deserializing)]
    pub has_default: bool
}

const DT_FIELD: [&'static str; 3] = ["date-time", "date", "time"];
impl ModelField{
    pub fn is_id_type(&self) -> bool{
        self.field_type.starts_with("id")
    }

    pub fn is_dt_type(&self) -> bool{
        DT_FIELD.contains(&self.field_type.as_str())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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

impl ModelRelation{
    pub fn single_belongs(&self) -> bool{
        self.rel_type.starts_with("one") &&
            self.keymaps.len()==1
    }

    pub fn relation_name(&self) -> String{
        format!("{}{}", self.title, self.rel_entity_name)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeyMap{
    #[serde(rename = "field-name", default)]
    pub field_name: String,
    #[serde(rename = "rel-field-name", default)]
    pub rel_field_name: String,
}

impl KeyMap{
    pub fn get_rel_field(&self) -> &str{
        if self.rel_field_name.is_empty(){
            self.field_name.as_str()
        }else{
            self.rel_field_name.as_str()
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ViewEntity{
    #[serde(rename = "entity-name", default)]
    pub entity_name: String,
    #[serde(default)]
    pub title: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct EntityModel{
    pub title: String,
    pub description: String,
    pub version: String,
    #[serde(rename = "default-resource-name", default)]
    pub default_resource_name: String,
    #[serde(rename = "entity", default)]
    pub entities: Vec<Entity>,
    #[serde(rename = "view-entity", default)]
    pub views: Vec<ViewEntity>
}

#[derive(Serialize, Deserialize)]
pub struct BelongsTo{
    pub field_name: String,
    pub model_name: String,
    pub rel_field_name: String,
    pub fk_name: String
}

impl EntityModel {
    pub fn get_entity(&self, name: &str) -> &Entity {
        self.entities.iter().find(|n|n.entity_name==name)
            .expect(format!("find entity {}", name).as_str())
    }

    pub fn build(&mut self){
        use std::collections::HashSet;
        let dt_types: HashSet<&'static str> =
            [ "date-time", "date", "time" ].iter().cloned().collect();
        for mut ent in &mut self.entities {
            for mut fld in &mut ent.fields {
                let is_pk=match ent.primary_keys.iter().map(|x| x.field_name.clone())
                    .find(|f|f==&fld.field_name) {
                    Some(_f) => true,
                    _ => false
                };
                fld.is_primary=is_pk;
                if dt_types.contains(fld.field_type.as_str()){
                    fld.has_default=false;
                }else{
                    fld.has_default=true;
                }
            }
            ent.multiple_keys=ent.primary_keys.len()>1;
        }
    }

    pub fn topo(&self) -> Vec<String>{
        let mut ts = TopologicalSort::<String>::new();
        let names=self.entity_names();
        for ent in &self.entities{
            let deps:Vec<String>=ent.belongs().iter().map(|e|e.model_name.clone()).collect();
            for belong in deps {
                if names.contains(&belong) {
                    ts.add_dependency(belong, &ent.entity_name);
                }
            }
        }
        let mut topo_stack:Vec<String>=Vec::new();
        while !ts.is_empty() {
            let mut result = ts.pop_all();
            if result.is_empty(){
                // If `pop_all` returns an empty vector and `len` is not 0, there is cyclic dependencies.
                warn!("has cyclic dependencies");
                break;
            }
            result.sort();
            topo_stack.append(&mut result);
        }

        let mut difference: Vec<_> = self.entity_names().into_iter()
            .filter(|item| !topo_stack.contains(item)).collect();
        topo_stack.append(&mut difference);

        topo_stack.reverse();
        topo_stack
    }

    pub fn entity_names(&self) -> Vec<String>{
        self.entities.iter().map(|e|e.entity_name.clone()).collect()
    }
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
    pub insert_type: String,
    #[serde(rename = "orig-type", default)]
    pub orig_type: String
}

#[derive(Debug, Deserialize)]
pub struct FieldTypes{
    #[serde(rename = "field-type-def", default)]
    pub field_types: Vec<FieldTypeDef>
}

impl FieldTypes{

    fn get_field(&self, field_type:&str) -> &FieldTypeDef{
        self.field_types.iter()
            .find(|x| x.field_type==field_type)
            .expect(format!("not found field type {}", field_type).as_str())
    }
    pub fn sql_type(&self, field_type: &str) -> String{
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
    pub fn orig_type(&self, field_type: &str) -> String{
        let typ=&self.get_field(field_type).orig_type;
        if typ.is_empty(){
            self.query_type(field_type)
        }else{
            typ.to_owned()
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

