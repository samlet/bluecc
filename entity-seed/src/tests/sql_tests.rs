use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};

#[tokio::test]
async fn async_works() -> std::io::Result<()> {
    assert_eq!(2 * 2, 4);
    Ok(())
}

async fn say_hello() {
    println!("Hello, world!");
}
#[tokio::test]
async fn await_works() {
    say_hello().await;

    let a = async { 1u8 };
    // let b = async { 2u8 };
    // assert_eq!(a.join(b).await, (1u8, 2u8))
    assert_eq!(a.await, 1u8)
}

#[test]
fn sqlite_works() {
    // let conn = SqliteConnection::connect("sqlite::memory:").await?;
}

#[tokio::test]
// async fn select_works() -> Result<(), quaint::error::Error> {
async fn select_works() -> anyhow::Result<()> {
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

