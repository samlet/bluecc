#![cfg_attr(debug_assertions, allow(dead_code, unused_imports))]
// #[macro_use]
// extern crate lazy_static;

// use std::env;
use structopt::StructOpt;
use std::collections::HashMap;
use entity_seed::tests::xml_tests::{FIELD_MAPPINGS, get_field_mappings, example_models, FieldTypes};

#[derive(StructOpt)]
struct Args {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt)]
enum Command {
    Gen { entity: String },
    List { },
}

/**
```bash
$ cargo run --bin sample gen Example
$ cargo run --bin sample gen ExampleStatus
$ cargo run --bin sample list
```
*/

#[async_std::main]
#[paw::main]
async fn main(args: Args) -> anyhow::Result<()> {
    match args.cmd {
        Some(Command::Gen { entity }) => {
            entity_gen_works(entity.as_str());
        }
        Some(Command::List {  }) => {
            println!("list all entities");
            let model=example_models();
            for ent in model.entities {
                println!("{}", ent.entity_name);
            }
        }
        None => {
            println!("Printing list of all todos");
            // list_todos(&pool).await?;
            entity_gen_works("Example");
        }
    }

    Ok(())
}

fn entity_gen_works(entity_name: &str) {
    use tera::{Result, Context, Filter, Function};
    use tera::Tera;
    use serde_json::{json, Value};

    struct SqlType;
    impl Filter for SqlType {
        fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            let val=FIELD_MAPPINGS.sql_type(value.as_str().unwrap());
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

    let model=example_models();
    let ent=model.get_entity(entity_name);
    assert_eq!(entity_name, ent.entity_name);

    let mut tera = Tera::default();
    tera.add_raw_template(
        "ent",
        r#"
CREATE TABLE {{ent['entity-name'] | snake_case -}} (
    id INTEGER PRIMARY KEY AUTO_INCREMENT,
{%- for fld in flds %}
    {{fld.name | snake_case}}: {{fld['type'] | sqltype}}{% if not loop.last %},{% endif %}
{%- endfor %}
);
        "#,
    )
        .unwrap();

    let mut context = Context::new();
    tera.register_filter("sqltype", SqlType);
    tera.register_filter("snake_case", snake_case);
    context.insert("ent", &ent);
    context.insert("flds", &ent.fields);
    let result = tera.render("ent", &context);
    println!("{}", result.unwrap());
}

