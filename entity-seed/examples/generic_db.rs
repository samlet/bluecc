use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};

#[tokio::main]
async fn main() -> Result<(), quaint::error::Error> {
    // let conn = Quaint::new("sqlite:///tmp/example.db").await?;
    let url="mysql://root:root@localhost:3306/ofbiz";
    let conn = Quaint::new(url).await?;

    let conditions = "product_id"
        .equals("WG-1111")
        .and("unit_price".less_than(100.00));
    let query = Select::from_table("order_item").so_that(conditions);
    // let result = conn.select(Select::default().value(1)).await?;
    let result = conn.select(query).await?;
    // println!("{:#?}", result);
    for row in result {
        let desc=row.get("STATUS_ID").unwrap();
        println!("{:?}, {:?}", row.get("ORDER_ID").unwrap().as_str(), desc.as_str());
    }

    Ok(())
}



