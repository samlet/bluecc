#[cfg(test)]
mod lib_tests {
    use super::*;
    use redis::Commands;
    use serde_json::json;
    use crate::xml_seed::SeedValue;

    #[test]
    fn tree_data_works() -> anyhow::Result<()> {
        let client = redis::Client::open("redis://127.0.0.1/")?;
        let mut con = client.get_connection()?;

        let key = "tree_obj_key";
        let tree_val="0 (1 2)";
        redis::cmd("tree.init").arg(key)
            .arg(tree_val)
            .query(&mut con)?;
        let tree_val="3 ( 4 5)";
        redis::cmd("tree.set_subtree").arg(key)
            .arg("2")
            .arg(tree_val)
            .query(&mut con)?;

        let val: String = redis::cmd("tree.get")
            .arg(key).query(&mut con)?;
        println!("New value: {}", val);
        assert_eq!("0( 1 2( 3( 4 5 ) ) )", val);

        Ok(())
    }
}

