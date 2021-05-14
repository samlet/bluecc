use crate::{GenericError, new_snowflake_id, get_entity_model, FIELD_MAPPINGS, Entity};
use std::collections::HashMap;
use crate::meta_model::BelongsTo;
use tera::Tera;
use tera::{Context, Filter, Function};
use serde_json::{json, Value};
use inflector::Inflector;

pub struct EntityGenerator{
    entities: Vec<String>,
    pub belongs_filter: bool,
    pub tera: Tera,
}

impl EntityGenerator {
    pub fn new(entities: Vec<String>) -> Self {
        let mut gen=EntityGenerator { entities, belongs_filter: true, tera: Tera::default() };
        gen.init().expect("init");
        gen
    }

    pub fn init(&mut self) -> Result<(), GenericError>{
        use tera::Result;

        struct SqlType;
        impl Filter for SqlType {
            fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
                let val = FIELD_MAPPINGS.sql_type(value.as_str().unwrap());
                Ok(Value::String(format!("{}", val)))
            }

            fn is_safe(&self) -> bool {
                true
            }
        }
        fn snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = inflector::cases::snakecase::to_snake_case(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }
        fn pascal_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = value.as_str().unwrap().to_pascal_case();
            Ok(Value::String(format!("{}", val)))
        }
        fn static_var(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = value.as_str().unwrap().to_screaming_snake_case();
            Ok(Value::String(format!("{}", val)))
        }
        fn query_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = FIELD_MAPPINGS.query_type(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }

        fn orig_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = FIELD_MAPPINGS.orig_type(value.as_str().unwrap());
            if val.starts_with("Option") {
                Ok(Value::String(format!("{}", val)))
            } else {
                Ok(Value::String(format!("Option<{}>", val)))
            }
        }

        fn opt_query_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = FIELD_MAPPINGS.query_type(value.as_str().unwrap());
            if val.starts_with("Option") {
                Ok(Value::String(format!("{}", val)))
            } else {
                Ok(Value::String(format!("Option<{}>", val)))
            }
        }
        fn insert_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val = FIELD_MAPPINGS.insert_type(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }
        fn fk_name(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            if value.as_str().unwrap() == "" {
                Ok(Value::String(format!("fk_{}", new_snowflake_id())))
            } else {
                Ok(value.clone())
            }
        }
        fn plain_type(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
            let mut val:String = FIELD_MAPPINGS.orig_type(value.as_str().unwrap());
            if val.starts_with("Option") {
                val=val.chars().skip_while(|&c|c!='<')
                    .skip(1)
                    .take_while(|&c|c!='>').collect();
            }
            Ok(Value::String(format!("{}", val)))
        }

        fn var_name(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
            let val = value.as_str().unwrap().to_camel_case();
            Ok(Value::String(format!("{}", val)))
        }

        fn pkg_name(value: &Value, _args: &HashMap<String, Value>) -> tera::Result<Value> {
            let val:String = value.as_str().unwrap()
                .trim_start_matches("org.apache.ofbiz.").to_string();
            Ok(Value::String(format!("{}", val)))
        }

        fn start_num(value: &Value, args: &HashMap<String, Value>) -> tera::Result<Value> {
            let base=args.get("base").expect("expect base parameter").as_i64().unwrap();
            let val:i64 = value.as_i64().unwrap()+base;
            Ok(Value::Number(val.into()))
        }

        self.tera.add_raw_template("ent", include_str!("incls/ent.j2"))?;
        self.tera.add_raw_template("ent_rel", include_str!("incls/ent_rel.j2"))?;
        self.tera.add_raw_template("ent_drop", include_str!("incls/ent_drop.j2"))?;
        self.tera.add_raw_template("model", include_str!("incls/model.j2"))?;
        self.tera.add_raw_template("dto", include_str!("incls/dto.j2"))?;
        self.tera.add_raw_template("dto_seed", include_str!("incls/dto_seed.j2"))?;
        self.tera.add_raw_template("dto_orig", include_str!("incls/dto_orig.j2"))?;
        self.tera.add_raw_template("dto_keys", include_str!("incls/dto_keys.j2"))?;

        /*
        #[serde(rename_all = "camelCase")]
        UserLogin {
            user_login_id: String,
            enabled: Option<String>,
            nick_name: Option<String>,
        },
         */
        self.tera.add_raw_template("enum", include_str!("incls/enum.j2"))?;

        self.tera.register_filter("pascal", pascal_case);

        self.tera.register_filter("sqltype", SqlType);
        self.tera.register_filter("query_type", query_type);
        self.tera.register_filter("opt_query_type", opt_query_type);
        self.tera.register_filter("orig_type", orig_type);
        self.tera.register_filter("insert_type", insert_type);
        self.tera.register_filter("snake_case", snake_case);
        self.tera.register_filter("static_var", static_var);
        self.tera.register_filter("fk", fk_name);
        self.tera.register_filter("plain_type", plain_type);
        self.tera.register_filter("var_name", var_name);
        self.tera.register_filter("pkg", pkg_name);
        self.tera.register_filter("start", start_num);

        Ok(())
    }

    pub fn entity_gen_works(&self, entity_name: &str, template_name: &str)
                        -> Result<String, GenericError> {
        // let model=&APP_CONTEXT.get_model(module);
        let ent = get_entity_model(entity_name)?;
        assert_eq!(entity_name, ent.entity_name);
        self.entity_gen_with(&ent, template_name)
    }

    pub fn entity_gen_with(&self, ent: &Entity, template_name: &str)
                        -> Result<String, GenericError> {
        // for f in &ent.fields {
        //     println!("* {}: {}", f.field_name, f.is_primary);
        // }

        let mut context = Context::new();
        context.insert("ent", &ent);
        context.insert("flds", &ent.fields.iter()
            .filter(|f| !f.is_primary).collect::<Vec<_>>());
        context.insert("keys", &ent.fields.iter()
            .filter(|f| f.is_primary).collect::<Vec<_>>());
        context.insert("multi_pk", &ent.multiple_keys);
        context.insert("pks", &ent.pks_str());

        let belongs = &ent.belongs();
        if !self.belongs_filter {
            context.insert("belongs", belongs);
            let has_rels = belongs.len() > 0;
            context.insert("has_rels", &has_rels);
        }else{
            let belongs=belongs.iter()
                .filter(|e|self.entities.contains(&e.model_name))
                .collect::<Vec<&BelongsTo>>();
            context.insert("belongs", &belongs);
            let has_rels = belongs.len() > 0;
            context.insert("has_rels", &has_rels);
        }

        let result = self.tera.render(template_name, &context)?;
        Ok(result)
    }

}

