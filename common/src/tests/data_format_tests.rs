use bson::Bson;
use serde_derive::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize)]
struct Person {
    name: String,
    age: i32,
    phones: Vec<String>,
}

// https://crates.io/crates/bson
#[test]
fn bson_works() {
    // Some BSON input data as a `Bson`.
    let bson_data: Bson = bson!({
        "name": "John Doe",
        "age": 43,
        "phones": [
            "+44 1234567",
            "+44 2345678"
        ]
    });
    // Deserialize the Person struct from the BSON data, automatically
    // verifying that the necessary keys are present and that they are of
    // the correct types.
    let person: Person = bson::from_bson(bson_data).unwrap();

    // Do things just like with any other Rust data structure.
    println!("Redacting {}'s record.", person.name);

    // person.name = "REDACTED".to_string();
    // Get a serialized version of the input data as a `Bson`.
    // let redacted_bson = bson::to_bson(&person).unwrap();
}

#[test]
fn url_join_works() -> anyhow::Result<()>{
    let this_document = Url::parse("http://servo.github.io/rust-url/url/index.html")?;
    let css_url = this_document.join("../main.css")?;
    assert_eq!(css_url.as_str(), "http://servo.github.io/rust-url/main.css");
    Ok(())
}

// https://docs.rs/url/2.2.1/url/
#[test]
fn url_works() -> anyhow::Result<()>{
    use url::{Url, Host, Position};
    let issue_list_url = Url::parse(
        "https://github.com/rust-lang/rust/issues?labels=E-easy&state=open"
    )?;

    assert!(issue_list_url.scheme() == "https");
    assert!(issue_list_url.username() == "");
    assert!(issue_list_url.password() == None);
    assert!(issue_list_url.host_str() == Some("github.com"));
    assert!(issue_list_url.host() == Some(Host::Domain("github.com")));
    assert!(issue_list_url.port() == None);
    assert!(issue_list_url.path() == "/rust-lang/rust/issues");
    assert!(issue_list_url.path_segments().map(|c| c.collect::<Vec<_>>()) ==
        Some(vec!["rust-lang", "rust", "issues"]));
    assert!(issue_list_url.query() == Some("labels=E-easy&state=open"));
    assert!(&issue_list_url[Position::BeforePath..] == "/rust-lang/rust/issues?labels=E-easy&state=open");
    assert!(issue_list_url.fragment() == None);
    assert!(!issue_list_url.cannot_be_a_base());

    Ok(())
}

use uuid::Uuid;

// https://crates.io/crates/uuid
#[test]
fn uuid_works() -> Result<(), uuid::Error> {
    let my_uuid =
        Uuid::parse_str("936DA01F9ABD4d9d80C702AF85C822A8")?;
    println!("{}", my_uuid.to_urn());

    let my_uuid = Uuid::new_v4();
    println!("{}, {}", my_uuid, my_uuid.to_simple());
    Ok(())
}