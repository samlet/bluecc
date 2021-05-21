use actix::prelude::*;

struct DocActor{
    client: redis::Client,
    con: redis::Connection,
}

impl DocActor{
    fn new() -> crate::Result<Self>{
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;
        Ok(DocActor{ client, con })
    }
}

impl Actor for DocActor {
    type Context = Context<Self>;
}

#[derive(Debug, PartialEq, MessageResponse)]
struct StringResp(String);

#[derive(Message)]
#[rtype(result = "StringResp")]
struct StringGetKey(String, String);
#[derive(Message)]
#[rtype(result = "()")]
struct StringSetKey(String, String, String);

impl Handler<StringGetKey> for DocActor {
    type Result = <StringGetKey as actix::Message>::Result;
    fn handle(&mut self, msg: StringGetKey, _: &mut Self::Context) -> StringResp {
        let val: String = redis::cmd("JSON.GET")
            .arg(msg.0).arg(msg.1).query(&mut self.con).expect("json.get");
        StringResp(val)
    }
}

impl Handler<StringSetKey> for DocActor {
    type Result = ();
    fn handle(&mut self, msg: StringSetKey, _: &mut Self::Context)  {
        let key=msg.0;
        let path=msg.1;
        let val=msg.2;
        redis::cmd("JSON.SET").arg(key).arg(path)
            .arg(val)
            .query::<()>(&mut self.con).expect("json.set");
    }
}

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

    #[test]
    fn test_doc_actor() -> anyhow::Result<()> {
        let key = "json_obj_key".to_string();
        System::new().block_on(async {
            let doc_actor = DocActor::new().unwrap().start();
            let res = doc_actor
                .send(StringGetKey(key, ".entity".to_string()))
                .await.unwrap();
            println!("{:?}", res);
        });

        Ok(())
    }

    #[test]
    fn test_doc_actor_setter() -> anyhow::Result<()> {
        let key = "json_obj_key";
        System::new().block_on(async {
            let doc_actor = DocActor::new().unwrap().start();
            doc_actor
                .send(StringSetKey(key.to_string(), ".entity".to_string(), "\"NewEnt\"".to_string()))
                .await.expect("set");
            let res = doc_actor
                .send(StringGetKey(key.to_string(), ".entity".to_string()))
                .await.expect("get");
            assert_eq!(StringResp("\"NewEnt\"".to_string()), res);
        });

        Ok(())
    }
}

