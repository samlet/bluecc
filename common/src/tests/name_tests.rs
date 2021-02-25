use inflector::cases::camelcase::to_camel_case;
use inflector::cases::snakecase::to_snake_case;

#[test]
fn camelcase_works() {
    let mock_string: &str = "FOO_BAR";
    let expected_string: String = "fooBar".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);

    let mock_string: &str = "foo-bar";
    let expected_string: String = "fooBar".to_string();
    let asserted_string: String = to_camel_case(mock_string);
    assert!(asserted_string == expected_string);
}

/// ref: https://docs.rs/Inflector/0.3.3/inflector/cases/snakecase/fn.to_snake_case.html
#[test]
fn snakecase_works() {
    let mock_string: &str = "Foo bar";
    let expected_string: String = "foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);

    let mock_string: &str  = "FooBar";
    let expected_string: String = "foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);

    let mock_string: &str  = "FOO_BAR";
    let expected_string: String = "foo_bar".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);

    let mock_string: &str  = "fooBar3";
    let expected_string: String = "foo_bar_3".to_string();
    let asserted_string: String = to_snake_case(mock_string);
    assert!(asserted_string == expected_string);
}

