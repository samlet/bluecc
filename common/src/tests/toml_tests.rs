use serde_derive::Deserialize;
use meval::{Expr, Context};

#[derive(Deserialize)]
struct Config {
    ip: String,
    port: Option<u16>,
    keys: Keys,
}

#[derive(Deserialize)]
struct Keys {
    github: String,
    travis: Option<String>,
}

#[test]
fn toml_works() {
    let config: Config = toml::from_str(r#"
        ip = '127.0.0.1'

        [keys]
        github = 'xxxxxxxxxxxxxxxxx'
        travis = 'yyyyyyyyyyyyyyyyy'
    "#).unwrap();

    assert_eq!(config.ip, "127.0.0.1");
    assert_eq!(config.port, None);
    assert_eq!(config.keys.github, "xxxxxxxxxxxxxxxxx");
    assert_eq!(config.keys.travis.as_ref().unwrap(), "yyyyyyyyyyyyyyyyy");
}

#[derive(Deserialize)]
struct Ode {
    #[serde(deserialize_with = "meval::de::as_f64")]
    x0: f64,
    #[serde(deserialize_with = "meval::de::as_f64")]
    t0: f64,
    f: Expr,
}

/// ref: https://github.com/rekka/meval-rs
#[test]
fn expr_toml_works() {
    let config = r#"
        x0 = "cos(1.)"
        t0 = 2
        f = "sin(x)"
    "#;
    let ode: Ode = toml::from_str(config).unwrap();

    assert_eq!(ode.x0, 1f64.cos());
    assert_eq!(ode.t0, 2f64);
    assert_eq!(ode.f.bind("x").unwrap()(2.), 2f64.sin());
}

