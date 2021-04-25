#[cfg(test)]
mod lib_tests {
    use super::*;
    use redis::Commands;
    use serde_json::json;
    use crate::xml_seed::SeedValue;

    #[test]
    fn scalar_data_works() -> anyhow::Result<()> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        let key = "the_key";
        con.set(key, 42)?;
        let val: isize = con.get(key)?;
        println!("New value: {}", val);

        Ok(())
    }

    #[test]
    fn json_data_works() -> anyhow::Result<()> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        let key = "json_key";

        let seed_val:SeedValue = serde_json::from_value(json!(
            {
                "entity": "ShoppingList",
                "values": {
                  "isPublic": "N",
                  "partyId": "DemoCustomer",
                  "shoppingListId": "DemoWishList",
                  "shoppingListTypeId": "SLT_WISH_LIST",
                  "productStoreId": "9000",
                  "currencyUom": "USD",
                  "listName": "Demo Wish List",
                  "isActive": "Y"
                }
              }
        ))?;

        con.set(key, serde_json::to_string_pretty(&seed_val)?)?;
        let val: String = con.get(key)?;
        println!("New value: {}", val);

        // with redis-json
        let key = "json_obj_key";
        redis::cmd("JSON.SET").arg(key).arg(".")
            .arg(serde_json::to_string_pretty(&seed_val)?)
            .query(&mut con)?;

        let val: String = redis::cmd("JSON.GET")
            .arg(key).arg(".").query(&mut con)?;
        println!("New value: {}", val);

        let val: String = redis::cmd("JSON.GET")
            .arg(key).arg(".entity").query(&mut con)?;
        println!("Entity name: {}", val);

        let val: String = redis::cmd("JSON.GET")
            .arg(key).arg(".values").query(&mut con)?;
        println!("Entity value: {}", val);

        Ok(())
    }
}

