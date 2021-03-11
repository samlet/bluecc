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

    pub async fn find_all(&self, entity_name: &str) -> Result<ResultSet, GenericError> {
        let query = Select::from_table(entity_name);
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
    use chrono::{DateTime, Utc};

    // source from: $ cargo run --bin seed gen security UserLogin dto_orig
    #[derive(Debug, Deserialize, Serialize, Clone)]
    #[serde(rename_all(serialize = "SCREAMING_SNAKE_CASE", deserialize = "SCREAMING_SNAKE_CASE"))]
    pub struct UserLogin{
        // keys
        pub user_login_id: Option<String>,
        // fields
        pub current_password: Option<String>,
        pub password_hint: Option<String>,
        pub is_system: Option<String>,
        pub enabled: Option<String>,
        pub has_logged_out: Option<String>,
        pub require_password_change: Option<String>,
        pub last_currency_uom: Option<String>,
        pub last_locale: Option<String>,
        pub last_time_zone: Option<String>,
        pub disabled_date_time: Option<chrono::NaiveDateTime>,
        pub successive_failed_logins: Option<i64>,
        pub external_auth_id: Option<String>,
        pub user_ldap_dn: Option<String>,
        pub disabled_by: Option<String>,

        // https://serde.rs/custom-date-format.html
        // DateTime supports Serde out of the box, but uses RFC3339 format. Provide
        // some custom logic to make it use our desired format.
        // #[serde(with = "cust_date_format")]
        pub created_tx_stamp: Option<DateTime<Utc>>,
    }

    mod cust_date_format {
        use chrono::{DateTime, Utc, TimeZone};
        use serde::{self, Deserialize, Serializer, Deserializer};

        const FORMAT: &'static str = "%Y-%m-%d %H:%M:%S%.f";

        // The signature of a serialize_with function must follow the pattern:
        //
        //    fn serialize<S>(&T, S) -> Result<S::Ok, S::Error>
        //    where
        //        S: Serializer
        //
        // although it may also be generic over the input types T.
        pub fn serialize<S>(
            date: &DateTime<Utc>,
            serializer: S,
        ) -> Result<S::Ok, S::Error>
            where
                S: Serializer,
        {
            let s = format!("{}", date.format(FORMAT));
            serializer.serialize_str(&s)
        }

        // The signature of a deserialize_with function must follow the pattern:
        //
        //    fn deserialize<'de, D>(D) -> Result<T, D::Error>
        //    where
        //        D: Deserializer<'de>
        //
        // although it may also be generic over the output types T.
        pub fn deserialize<'de, D>(
            deserializer: D,
        ) -> Result<DateTime<Utc>, D::Error>
            where
                D: Deserializer<'de>,
        {
            let s = String::deserialize(deserializer)?;
            Utc.datetime_from_str(&s, FORMAT).map_err(serde::de::Error::custom)
        }
    }

    #[tokio::test]
    async fn serialize_json_works() -> anyhow::Result<()> {
        let delegator=Delegator::new().await?;
        let rs=delegator.find_all("user_login").await?;
        let jval=serde_json::Value::from(rs);
        let rows=jval.as_array();
        println!("total {}", rows.unwrap().len());
        for row in rows.unwrap() {
            println!("{:?}", row);
            let v=serde_json::from_value::<UserLogin>(row.to_owned())?;
            println!("{:?}", v);
        }
        Ok(())
    }

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

    #[tokio::test]
    async fn visit_works() -> anyhow::Result<()> {
        use quaint::{visitor::{Visitor, Postgres}};
        let query = Select::from_table(("crm", "users"));
        let (sql, _) = Postgres::build(query)?;
        assert_eq!("SELECT `crm`.`users`.* FROM `crm`.`users`".replace("`","\""), sql);

        //

        let join = "dogs".on(("dogs", "slave_id").equals(Column::from(("cats", "master_id"))));
        let query = Select::from_table("cats")
            .value(Table::from("cats").asterisk())
            .value(col!("dogs", "age") - val!(4))
            .inner_join(join);

        let (sql, params) = Postgres::build(query)?;

        assert_eq!(
            "SELECT `cats`.*, (`dogs`.`age` - $1) FROM `cats` INNER JOIN `dogs` ON `dogs`.`slave_id` = `cats`.`master_id`".replace("`","\""),
            sql
        );
        assert_eq!(vec![Value::from(4)], params);

        Ok(())
    }
}

