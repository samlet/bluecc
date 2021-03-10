use std::env;
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

async fn quaint_conn() -> quaint::Result<Quaint>{
    dotenv::dotenv().ok();
    let url = env::var("OFBIZ_URL").expect("OFBIZ_URL env not set");
    Quaint::new(url.as_str()).await
}

#[tokio::test]
async fn insert_works() -> anyhow::Result<()> {
    // let url="mysql://root:root@localhost/todos";
    let url="postgres://xiaofeiwu:@localhost:5432/seed";
    let conn = Quaint::new(url).await?;

    // let conn=quaint_conn().await?;
    let table="books";
    let insert = Insert::multi_into(table, vec!["id", "author"])
        .values(vec![Value::integer(1), Value::text("Musti")])
        .values(vec![Value::integer(2), Value::text("Naukio")])
        .values(vec![Value::integer(3), Value::text("Belka")]);

    conn.insert(insert.into()).await?;

    let query = Select::from_table(table).limit(1).offset(2);

    let res = conn.select(query).await?;
    assert_eq!(1, res.len());

    let row = res.get(0).unwrap();
    assert_eq!(Some("Belka"), row["author"].as_str());

    Ok(())
}

#[tokio::test]
// async fn select_works() -> Result<(), quaint::error::Error> {
async fn select_works() -> anyhow::Result<()> {
    // let conn = Quaint::new("sqlite:///tmp/example.db").await?;
    let url="mysql://root:root@localhost:3306/ofbiz";
    let conn = Quaint::new(url).await?;
    // let conn=quaint_conn().await?;  // 切换至函数形式, 会导致测试变慢

    let conditions = "product_id"
        .equals("WG-1111")
        .and("unit_price".less_than(100.00));
    let query = Select::from_table("order_item").so_that(conditions);
    // let result = conn.select(Select::default().value(1)).await?;
    let result = conn.select(query).await?;

    let cols=result.columns();
    println!("cols (total {}) {:?}", cols.len(), cols);
    // println!("{:#?}", result);
    for row in result {
        let desc=row.get("STATUS_ID").unwrap();
        println!("{:?}, {:?}", row.get("ORDER_ID").unwrap().as_str(), desc.as_str());
    }

    Ok(())
}

