#[cfg(test)]
mod lib_tests {
    use super::*;
    use std::collections::{BTreeMap, HashMap};
    use tera::{Result, Context, Filter, Function};
    use tera::Tera;
    use serde_json::{json, Value, from_value, to_value};

    // tera.register_function("url_for", make_url_for(urls));
    // {{ url_for(name="home") }}
    fn make_url_for(urls: BTreeMap<String, String>) -> Box<dyn Function> {
        Box::new(move |args:&HashMap<String, Value>| -> Result<Value> {
            match args.get("name") {
                Some(val) => match from_value::<String>(val.clone()) {
                    Ok(v) => Ok(to_value(urls.get(&v).unwrap()).unwrap()),
                    Err(_) => Err("oops".into()),
                },
                None => Err("oops".into()),
            }
        })
    }

    #[test]
    fn fn_works() -> anyhow::Result<()> {
        let mut movie_reviews = BTreeMap::new();

        // review some movies.
        movie_reviews.insert("Office Space".to_owned(),
                             "Deals with real issues in the workplace.".to_owned());
        movie_reviews.insert("Pulp Fiction".to_owned(), "Masterpiece.".to_owned());
        movie_reviews.insert("The Godfather".to_owned(), "Very enjoyable.".to_owned());
        movie_reviews.insert("The Blues Brothers".to_owned(), "Eye lyked it a lot.".to_owned());

        let mut _tera = Tera::default();
        // tera.register_function("url_for", &*make_url_for(movie_reviews));
        // let mut context = Context::new();
        // let t = Tera::one_off("{{ url_for(name=\"Office Space\") }}", &context, true)?;
        // println!("{}", t);
        Ok(())
    }
}


