#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
#[macro_use]
extern crate lazy_static;

// use std::env;
use structopt::StructOpt;
use std::collections::HashMap;
use entity_seed::tests::app_context::{APP_CONTEXT};
use std::path::PathBuf;
use entity_seed::meta_model::ModelField;

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
    Gen { entity: String, type_name:String },
    List { },
}

/**
```bash
$ cargo run --bin sample gen Example ent
$ cargo run --bin sample gen ExampleStatus dto
$ cargo run --bin sample list
$ cargo run --bin sample -- -o out_file list
```
*/

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> anyhow::Result<()> {
    use tempfile::Builder as TempfileBuilder;

    let tempfile = TempfileBuilder::new().tempfile_in("./")?;
    println!(".. generate files .. to {}", args.output
        .or(Some(PathBuf::from(tempfile.path())))
        .unwrap().display());

    match args.cmd {
        Some(Command::Gen { entity, type_name }) => {
            entity_gen_works(entity.as_str(), type_name.as_str());
        }
        Some(Command::List {  }) => {
            println!("list all entities");
            let model=&APP_CONTEXT.models;
            for ent in &model.entities {
                println!("{}", ent.entity_name);
            }
        }
        None => {
            println!(".. model Example");
            entity_gen_works("Example", "ent");
            entity_gen_works("Example", "model");
            entity_gen_works("Example", "dto");

            println!(".. model ExampleItem");
            entity_gen_works("ExampleItem", "ent");
            entity_gen_works("ExampleItem", "model");
            entity_gen_works("ExampleItem", "dto");
        }
    }

    Ok(())
}

fn entity_gen_works(entity_name: &str, template_name: &str) {
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

    let model=&APP_CONTEXT.models;
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

{% for item in belongs %}
ALTER TABLE {{ent['entity-name'] | snake_case }} ADD CONSTRAINT {{item.fk_name}}
    FOREIGN KEY ({{item.field_name}}) REFERENCES {{item.model_name | snake_case -}} ({{item.rel_field_name}});
{%- endfor %}
        "#,
    )
        .unwrap();

    tera.add_raw_template(
        "model",
        r#"
#[derive(Debug, Queryable, Identifiable, Associations)]
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
pub struct {{ent['entity-name'] -}} {
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
    context.insert("ent", &ent);
    context.insert("flds", &ent.fields.iter()
        .filter(|f| !f.is_primary).collect::<Vec<_>>());
    context.insert("keys", &ent.fields.iter()
        .filter(|f| f.is_primary).collect::<Vec<_>>());
    context.insert("multi_pk", &ent.multiple_keys);
    context.insert("pks", &ent.pks_str());
    context.insert("belongs", &ent.belongs());

    let result = tera.render(template_name, &context);
    println!("{}", result.unwrap());
}

