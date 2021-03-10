use std::env;
use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};
use crate::GenericError;

struct Delegator{
    conn: Quaint
}

impl Delegator{
    pub async fn new() -> Result<Self, GenericError> {
        let url = "mysql://root:root@localhost:3306/ofbiz";
        // dotenv::dotenv().ok();
        // let url = env::var("OFBIZ_URL").unwrap();
        Ok(Delegator { conn: (Quaint::new(url).await?) })
    }

    pub async fn find(&self, entity_name: &str, conditions: ConditionTree<'_>) -> Result<ResultSet, GenericError> {
        let query = Select::from_table(entity_name).so_that(conditions);
        let result = self.conn.select(query).await?;
        Ok(result)
    }
}

pub async fn result_str(rs: ResultSet) -> String {
    let jval=serde_json::Value::from(rs);
    serde_json::to_string_pretty(&jval).expect("pretty json")
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use serde_json::to_string_pretty;

    #[tokio::test]
    async fn delegator_works() -> anyhow::Result<()> {
        let delegator=Delegator::new().await?;
        let conditions = "product_id"
            .equals("WG-1111")
            .and("unit_price".less_than(100.00));
        let result=delegator.find("ORDER_ITEM", conditions).await?;

        let cols = result.columns();
        println!("cols (total {}) {:?}", cols.len(), cols);

        println!("{}", result_str(result).await); // 必须加await, 否则会导致测试运行延迟
        // let jval=serde_json::Value::from(result);
        // println!("{}", to_string_pretty(&jval)?);

        // for row in result {
        //     let desc = row.get("STATUS_ID").unwrap();
        //     println!("{:?}, {:?}",
        //              row.get("ORDER_ID").unwrap().as_str().unwrap(),
        //              desc.as_str().unwrap());
        // }

        Ok(())
    }

    #[tokio::test]
    async fn select_works() -> anyhow::Result<()> {
        // let conn = Quaint::new("sqlite:///tmp/example.db").await?;
        let url = "mysql://root:root@localhost:3306/ofbiz";
        let conn = Quaint::new(url).await?;
        // let conn=quaint_conn().await?;  // 切换至函数形式, 会导致测试变慢

        let conditions = "product_id"
            .equals("WG-1111")
            .and("unit_price".less_than(100.00));
        let query = Select::from_table("order_item").so_that(conditions);
        // let result = conn.select(Select::default().value(1)).await?;
        let result = conn.select(query).await?;

        let cols = result.columns();
        println!("cols (total {}) {:?}", cols.len(), cols);
        // println!("{:#?}", result);
        for row in result {
            let desc = row.get("STATUS_ID").unwrap();
            println!("{:?}, {:?}", row.get("ORDER_ID").unwrap().as_str(), desc.as_str());
        }

        Ok(())
    }
}

