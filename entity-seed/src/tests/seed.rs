#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[macro_use]
extern crate lazy_static;

// use std::env;
use structopt::StructOpt;
use std::collections::HashMap;
use entity_seed::tests::app_context::{APP_CONTEXT};
use std::path::PathBuf;
use entity_seed::meta_model::{ModelField, EntityModel};
use entity_seed::tests::seed_conf::SeedConfig;
use entity_seed::snowflake::new_snowflake_id;

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
    /// Output file, stdout if not present
    #[structopt(short = "o", long = "output", parse(from_os_str))]
    output: Option<PathBuf>,
}

#[derive(StructOpt)]
enum Command {
    Gen { module: String, entity: String, type_name:String },
    All { module: String},
    List { module: String},
}

/**
```bash
$ cargo run --bin seed gen Example ent
$ cargo run --bin seed gen ExampleStatus dto
$ cargo run --bin seed list security
$ cargo run --bin seed all security
# $ cargo run --bin seed -- -o out_file list
```
*/

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> anyhow::Result<()> {
    // use tempfile::Builder as TempfileBuilder;
    // let tempfile = TempfileBuilder::new().tempfile_in("./")?;
    //
    // println!(".. generate files .. to {}", args.output
    //     .or(Some(PathBuf::from(tempfile.path())))
    //     .unwrap().display());

    match args.cmd {
        Some(Command::Gen { module, entity, type_name }) => {
            entity_gen_works(module.as_str(), entity.as_str(), type_name.as_str())
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
        }
        Some(Command::List { module  }) => {
            let model=&APP_CONTEXT.get_model(module.as_str());
            println!("list all entities");
            for ent in &model.entities {
                println!("{}", ent.entity_name);
            }
        }
        Some(Command::All { module }) => {
            let model=&APP_CONTEXT.get_model(module.as_str());
            let conf=SeedConfig::load()?;
            let module_conf=conf.module_conf(module.as_str()).unwrap();
            let model_file=&module_conf.model;
            let up_sql_file=&module_conf.up_sql;
            let down_sql_file=&module_conf.down_sql;
            println!("generate all entities to {} ..", model_file);

            let header=conf.get_header(module.as_str());
            let gen = |typs:Vec<&str>, write_header:bool|  {
                let mut output=String::new();
                if write_header {
                    output.push_str(header.as_str());
                }
                for typ in typs {
                    let mut ents:Vec<String>= model.entities.iter()
                        .map(|x|x.entity_name.clone()).collect::<Vec<String>>();
                    if typ=="ent_drop"{
                        // ents.reverse();
                        ents=model.topo();
                    }
                    for ent in &ents {
                        println!("generate {} for {}", ent, typ);

                        let cnt: String = entity_gen_works(module.as_str(), ent.as_str(), typ).unwrap();
                        output.push_str(cnt.as_str());
                    }
                }
                output
            };

            std::fs::write(model_file, gen(vec!["model"], true))?;
            std::fs::write(up_sql_file, gen(vec!["ent", "ent_rel"], false))?;
            std::fs::write(down_sql_file, gen(vec!["ent_drop"], false))?;
            println!("done.");
        }

        None => {
            let module="example";
            println!(".. model Example");
            entity_gen_works(module, "Example", "ent")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
            entity_gen_works(module, "Example", "model")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
            entity_gen_works(module, "Example", "dto")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();

            println!(".. model ExampleItem");
            entity_gen_works(module, "ExampleItem", "ent")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
            entity_gen_works(module, "ExampleItem", "model")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
            entity_gen_works(module, "ExampleItem", "dto")
                .and_then(|x| {println!("{}", x); Ok(())}).ok();
        }
    }

    Ok(())
}

fn entity_gen_works(module:&str, entity_name: &str, template_name: &str) -> tera::Result<String> {
    use tera::{Result, Context, Filter, Function};
    use tera::Tera;
    use serde_json::{json, Value};

    struct SqlType;
    impl Filter for SqlType {
        fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val=APP_CONTEXT.field_mappings.sql_type(value.as_str().unwrap());
            Ok(Value::String(format!("{}", val)))
        }

        fn is_safe(&self) -> bool {
            true
        }
    }
    fn snake_case(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=inflector::cases::snakecase::to_snake_case(value.as_str().unwrap());
        Ok(Value::String(format!("{}", val)))
    }
    fn query_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=APP_CONTEXT.field_mappings.query_type(value.as_str().unwrap());
        Ok(Value::String(format!("{}", val)))
    }
    fn insert_type(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        let val=APP_CONTEXT.field_mappings.insert_type(value.as_str().unwrap());
        Ok(Value::String(format!("{}", val)))
    }
    fn fk_name(value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
        if value.as_str().unwrap()=="" {
            Ok(Value::String(format!("fk_{}", new_snowflake_id())))
        }else{
            Ok(value.clone())
        }
    }

    let model=&APP_CONTEXT.get_model(module);
    let ent=model.get_entity(entity_name);
    assert_eq!(entity_name, ent.entity_name);
    // for f in &ent.fields {
    //     println!("* {}: {}", f.field_name, f.is_primary);
    // }

    let mut tera = Tera::default();
    tera.add_raw_template(
        "ent",
        r#"
CREATE TABLE {{ent['entity-name'] | snake_case -}} (
{%- for fld in flds %}
    {{fld.name | snake_case}} {{fld['type'] | sqltype}},
{%- endfor %}
{% if ent.multiple_keys %}
{%- for fld in keys %}
    {{fld.name | snake_case}} {{fld['type'] | sqltype}},
{%- endfor %}
{%- endif %}
{%- if not ent.multiple_keys %}
    {{pks}} SERIAL PRIMARY KEY
{%- else %}
    PRIMARY KEY ({{pks}})
{%- endif %}
);
        "#,
    )
        .unwrap();

    tera.add_raw_template(
        "ent_rel",
        r#"
{% for item in belongs %}
ALTER TABLE {{ent['entity-name'] | snake_case }} ADD CONSTRAINT {{item.fk_name | fk}}
    FOREIGN KEY ({{item.field_name}}) REFERENCES {{item.model_name | snake_case -}} ({{item.rel_field_name}});
{%- endfor %}
        "#,
    )
        .unwrap();
    tera.add_raw_template(
        "ent_drop",
        r#"
DROP TABLE {{ent['entity-name'] | snake_case }};
        "#,
    )
        .unwrap();
    tera.add_raw_template(
        "model",
        r#"
#[derive(Debug, Queryable, Identifiable{% if has_rels %}, Associations{% endif %})]
#[primary_key({{pks}})]
{%- for item in belongs %}
#[belongs_to({{item.model_name}}, foreign_key = "{{item.field_name}}")]
{%- endfor %}
#[table_name = "{{ent['entity-name'] | snake_case}}"]
pub struct {{ent['entity-name'] -}} {
    // keys
{%- for fld in keys %}
    pub {{fld.name | snake_case}}: {{fld['type'] | query_type}},
{%- endfor %}
    // fields
{%- for fld in flds %}
    pub {{fld.name | snake_case}}: {{fld['type'] | query_type}}{% if not loop.last %},{% endif %}
{%- endfor %}
}
        "#,
    )
        .unwrap();

    tera.add_raw_template(
        "dto",
        r#"
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct {{ent['entity-name'] -}}<'a> {
    // keys
{%- for fld in keys %}
    #[serde(rename = "{{fld.name}}"{% if fld.has_default %}, default{% endif %})]
    pub {{fld.name | snake_case}}: {{fld['type'] | insert_type}},
{%- endfor %}
    // fields
{%- for fld in flds %}
    #[serde(rename = "{{fld.name}}"{% if fld.has_default %}, default{% endif %})]
    pub {{fld.name | snake_case}}: {{fld['type'] | insert_type}}{% if not loop.last %},{% endif %}
{%- endfor %}
}
        "#,
    )
        .unwrap();

    let mut context = Context::new();
    tera.register_filter("sqltype", SqlType);
    tera.register_filter("query_type", query_type);
    tera.register_filter("insert_type", insert_type);
    tera.register_filter("snake_case", snake_case);
    tera.register_filter("fk", fk_name);
    context.insert("ent", &ent);
    context.insert("flds", &ent.fields.iter()
        .filter(|f| !f.is_primary).collect::<Vec<_>>());
    context.insert("keys", &ent.fields.iter()
        .filter(|f| f.is_primary).collect::<Vec<_>>());
    context.insert("multi_pk", &ent.multiple_keys);
    context.insert("pks", &ent.pks_str());
    let belongs=&ent.belongs();
    context.insert("belongs", &belongs);
    let has_rels=belongs.len()>0;
    context.insert("has_rels", &has_rels);

    let result = tera.render(template_name, &context);
    // println!("{}", result.unwrap());
    result
}

