use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};

#[tokio::main]
async fn main() -> Result<(), quaint::error::Error> {
    // let conn = Quaint::new("sqlite:///tmp/example.db").await?;
    let url="mysql://root:root@localhost:3306/todos";
    let conn = Quaint::new(url).await?;

    let conditions = "description"
        .equals("todo description")
        .and("done".less_than(10));
    let query = Select::from_table("todos").so_that(conditions);
    // let result = conn.select(Select::default().value(1)).await?;
    let result = conn.select(query).await?;
    for row in result {
        let desc=row.get("description").unwrap();
        println!("{:?}, {:?}", row.get("id").unwrap().as_i64(), desc.as_str());
    }

    Ok(())
}

