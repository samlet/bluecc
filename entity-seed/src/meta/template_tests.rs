use serde_derive::Serialize;
use std::collections::{BTreeMap, HashMap};
use std::error::Error;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use lazy_static::lazy_static;
use serde_json::{json, Value};
use tera::{Result, Context, Filter, Function};
use tera::Tera;
// use crate::meta::xml_tests::FieldTypeDef;

fn render_template(content: &str, context: &Context) -> Result<String> {
    let mut tera = Tera::default();
    tera.add_raw_template("hello.html", content).unwrap();
    tera.register_function("get_number", |_: &HashMap<String, Value>| Ok(Value::Number(10.into())));
    tera.register_function("get_true", |_: &HashMap<String, Value>| Ok(Value::Bool(true.into())));
    tera.register_function("get_string", |_: &HashMap<String, Value>| {
        Ok(Value::String("Hello".to_string()))
    });

    tera.render("hello.html", context)
}

#[allow(dead_code)]
#[derive(Debug, Serialize)]
pub struct NestedObject {
    pub label: String,
    pub parent: Option<Box<NestedObject>>,
    pub numbers: Vec<usize>,
}

#[derive(Debug, Serialize)]
pub struct Review {
    title: String,
    paragraphs: Vec<String>,
}

impl Review {
    #[allow(dead_code)]
    pub fn new() -> Review {
        Review {
            title: "My review".to_owned(),
            paragraphs: vec!["A".to_owned(), "B".to_owned(), "C".to_owned()],
        }
    }
}


#[test]
fn render_simple_string() {
    let result = render_template("<h1>Hello world</h1>", &Context::new());
    assert_eq!(result.unwrap(), "<h1>Hello world</h1>".to_owned());
}

#[test]
fn does_render_owned_for_loop_with_objects() {
    let mut context = Context::new();
    let data = json!([
        {"id": 1, "year": 2015},
        {"id": 2, "year": 2015},
        {"id": 3, "year": 2016},
        {"id": 4, "year": 2017},
        {"id": 5, "year": 2017},
        {"id": 6, "year": 2017},
        {"id": 7, "year": 2018},
        {"id": 8},
        {"id": 9, "year": null},
    ]);
    context.insert("something", &data);

    let tpl =
        r#"{% for year, things in something | group_by(attribute="year") %}{{year}},{% endfor %}"#;
    let expected = "2015,2016,2017,2018,";
    assert_eq!(render_template(tpl, &context).unwrap(), expected);
}


// https://github.com/Keats/tera/issues/342
#[test]
fn redefining_loop_value_doesnt_break_loop() {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "tpl",
        r#"
{%- set string = "abcdefghdijklm" | split(pat="d") -%}
{% for i in string -%}
    {%- set j = i ~ "lol" ~ " " -%}
    {{ j }}
{%- endfor -%}
        "#,
    )
        .unwrap();
    let context = Context::new();
    let result = tera.render("tpl", &context);

    assert_eq!(result.unwrap(), "abclol efghlol ijklmlol ");
}

#[test]
fn map_works() {
    let mut context = Context::new();
    let mut map = BTreeMap::new();
    map.insert("name", "bob");
    map.insert("age", "18");
    context.insert("map", &map);
    let input=r#"
    {% for key, val in map %}{{key}}:{{val}} {% endfor %}
    "#;
    let result= render_template(input, &context).unwrap();
    println!("{}", result);
}


#[test]
fn object_works() {
    let mut tera = Tera::default();
    tera.add_raw_template(
        "ent",
        r#"
{% for fld in authors -%}
    {{ fld.id }},
{%- endfor -%}
        "#,
    )
        .unwrap();

    #[derive(Debug, Serialize)]
    struct Author {
        id: u8,
    }

    let mut context = Context::new();
    context.insert("authors", &vec![Author { id: 1 }, Author { id: 2 }, Author { id: 3 }]);
    let result = tera.render("ent", &context);
    println!("{}", result.unwrap());
}

// https://github.com/Keats/tera/issues/422
#[test]
fn default_filter_works_in_condition() {
    let mut tera = Tera::default();
    tera.add_raw_template("test.html", r#"{% if frobnicate|default(value=True) %}here{% endif %}"#)
        .unwrap();
    let res = tera.render("test.html", &Context::new());
    assert_eq!(res.unwrap(), "here");
}


// https://github.com/Keats/tera/issues/185
#[test]
fn ok_many_variable_blocks() {
    let mut context = Context::new();
    context.insert("username", &"bob");

    let mut tpl = String::new();
    for _ in 0..200 {
        tpl.push_str("{{ username }}")
    }
    let mut expected = String::new();
    for _ in 0..200 {
        expected.push_str("bob")
    }
    assert_eq!(render_template(&tpl, &context).unwrap(), expected);
}


#[test]
fn safe_filter_works() {
    struct Safe;
    impl Filter for Safe {
        fn filter(&self, value: &Value, _args: &HashMap<String, Value>) -> Result<Value> {
            Ok(Value::String(format!("<div>{}</div>", value.as_str().unwrap())))
        }

        fn is_safe(&self) -> bool {
            true
        }
    }

    let mut tera = Tera::default();
    tera.register_filter("safe_filter", Safe);
    tera.add_raw_template("test.html", r#"{{ "Hello" | safe_filter }}"#).unwrap();

    let res = tera.render("test.html", &Context::new());
    assert_eq!(res.unwrap(), "<div>Hello</div>");
}

#[test]
fn safe_function_works() {
    struct Safe;
    impl Function for Safe {
        fn call(&self, _args: &HashMap<String, Value>) -> Result<Value> {
            Ok(Value::String("<div>Hello</div>".to_owned()))
        }

        fn is_safe(&self) -> bool {
            true
        }
    }

    let mut tera = Tera::default();
    tera.register_function("safe_function", Safe);
    tera.add_raw_template("test.html", "{{ safe_function() }}").unwrap();

    let res = tera.render("test.html", &Context::new());
    assert_eq!(res.unwrap(), "<div>Hello</div>");
}

#[test]
fn serialize_object_works() {
    #[derive(Debug, Serialize)]
    pub struct FieldTypeDef{
        pub field_type: String,
        pub sql_type: String,
        #[serde(rename = "java-type", default)]
        pub java_type: String
    }

    let ft=FieldTypeDef{
        field_type: "id".to_string(),
        sql_type: "string".to_string(),
        java_type: "String".to_string()
    };

    let mut tera = Tera::default();
    tera.add_raw_template(
        "ent",
        r#"
{% for fld in flds -%}
    {{fld.sql_type}}: {{ fld.field_type }} ({{fld['java-type']}})
{%- endfor -%}
        "#,
    )
        .unwrap();

    let mut context = Context::new();
    context.insert("flds", &vec![ft]);
    let result = tera.render("ent", &context);
    println!("{}", result.unwrap());

}


#[derive(Serialize)]
struct Test {
    a: String,
    b: String,
    c: Vec<String>,
}

#[test]
fn var_access_by_square_brackets_errors() {
    let mut context = Context::new();
    context.insert("var", &Test { a: "hi".into(), b: "there".into(), c: vec![] });
    let t = Tera::one_off("{{var[csd]}}", &context, true);
    assert!(t.is_err(), "Access of csd should be impossible");
}


#[test]
fn var_access_by_square_brackets() {
    let mut context = Context::new();
    context.insert(
        "var",
        &Test { a: "hi".into(), b: "i_am_actually_b".into(), c: vec!["fred".into()] },
    );
    context.insert("zero", &0);
    context.insert("a", "b");

    let mut map = HashMap::new();
    map.insert("true", "yes");
    map.insert("false", "no");
    map.insert("with space", "works");
    map.insert("with/slash", "works");
    let mut deep_map = HashMap::new();
    deep_map.insert("inner_map", &map);
    context.insert("map", &map);
    context.insert("deep_map", &deep_map);
    context.insert("bool_vec", &vec!["true", "false"]);

    let inputs = vec![
        ("{{var.a}}", "hi"),
        ("{{var['a']}}", "hi"),
        ("{{var[\"a\"]}}", "hi"),
        ("{{var['c'][0]}}", "fred"),
        ("{{var['c'][zero]}}", "fred"),
        ("{{var[a]}}", "i_am_actually_b"),
        ("{{map['with space']}}", "works"),
        ("{{map['with/slash']}}", "works"),
        ("{{deep_map['inner_map'][bool_vec[zero]]}}", "yes"),
    ];

    for (input, expected) in inputs {
        println!("{:?} -> {:?}", input, expected);
        assert_eq!(Tera::one_off(input, &context, true).unwrap(), expected);
    }
}

#[test]
fn one_off_works() -> anyhow::Result<()> {
    let mut context = Context::new();
    context.insert("greeting", &"hello");
    let r=Tera::one_off("{{ greeting }} world", &context, true)?;
    println!("{}", r);
    Ok(())
}