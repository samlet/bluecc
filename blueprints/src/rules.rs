#[cfg(test)]
mod lib_tests {
    use super::*;

    use rules::{Engine, Rule, Map, from_dynamic};
    use serde_json::json;
    use serde::{Serialize, Deserialize};
    use common::prelude::pretty;

    #[derive(Deserialize, Serialize)]
    struct Facts {
        name: String,
        age: u8,
        action: String,
    }

    fn age_greater_than20_less_than_inclusive25(p: Map) -> bool {
        let facts: Facts = from_dynamic(&p.into()).unwrap();
        facts.age > 20 && facts.age <= 25
    }

    #[tokio::test]
    async fn rule_works() -> anyhow::Result<() > {
        let rule_json = json !({
        "conditions": {
            "and": [
                {"field": "name","operator": "string_equals","value": "Cheng JIANG"},
                {"field": "age","operator": "int_in_range","value": [20, 25]},
                {
                    "and": [
                        {"expr": "facts.age > 20 && facts.age <= 25",},
                        {"expr": "my_function(facts)",},
                    ]
                },
                {"field": "action","operator": "string_equals","value": "coding in rust"}
            ]
        },
        "events": [
            {
                "type": "post_to_callback_url",
                "params": {
                    "callback_url": "http://example.com/peoples/conding_in_rust",
                    "type": "info",
                    "title": "Another person is coding in rust",
                    "message": "Name: {{ name }}, Age: {{ age }}, Action: {{ action }}"
                }
            }
        ]
        });

        let rule: Rule = serde_json::from_str::<Rule>( &serde_json::to_string(&rule_json)?)?;

        let mut engine = Engine::new();
        engine.add_rule(rule);
        engine.add_function("my_function", age_greater_than20_less_than_inclusive25);

        let facts = json ! ({
            "name": "Cheng JIANG",
            "age": 24,
            "action": "coding in rust",
        });

        let rule_results = engine.run( &facts).await?;
        println ! ("{}", pretty(&rule_results));

        Ok(())
    }
}

