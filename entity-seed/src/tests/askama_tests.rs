use askama::Template;
#[cfg(feature = "serde-json")]
use serde_json::Value;

#[derive(Template)]
#[template(source = "{{ s|myfilter }}", ext = "txt")]
struct MyFilterTemplate<'a> {
    s: &'a str,
}

mod filters {
    pub fn myfilter(s: &str) -> ::askama::Result<String> {
        Ok(s.replace("oo", "aa"))
    }
    // for test_nested_filter_ref
    pub fn mytrim(s: &dyn (::std::fmt::Display)) -> ::askama::Result<String> {
        let s = format!("{}", s);
        Ok(s.trim().to_owned())
    }
}

#[test]
fn test_my_filter() {
    let t = MyFilterTemplate { s: "foo" };
    assert_eq!(t.render().unwrap(), "faa");
}

#[derive(Template)]
#[template(path = "filters.html")]
struct TestTemplate {
    strvar: String,
}

#[test]
fn filter_escape() {
    let s = TestTemplate {
        strvar: "// my <html> is \"unsafe\" & should be 'escaped'".to_string(),
    };
    assert_eq!(
        s.render().unwrap(),
        "&#x2f;&#x2f; my &lt;html&gt; is &quot;unsafe&quot; &amp; \
         should be &#x27;escaped&#x27;"
    );
}

#[test]
fn filter_fmt() {
    #[derive(Template)]
    #[template(source = "{{ var|fmt(\"{:?}\") }}", ext = "html", escape = "none")]
    struct FmtTemplate<'a> {
        var: &'a str,
    }
    let t = FmtTemplate { var: "formatted" };
    assert_eq!(t.render().unwrap(), "\"formatted\"");
}