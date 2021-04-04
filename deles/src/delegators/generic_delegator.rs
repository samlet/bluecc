use std::env;
use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};
use seed::GenericError;
use inflector::Inflector;
use serde::de::DeserializeOwned;
use std::collections::HashMap;
use crate::delegators::values::{get_values_from_map, get_values_from_string_map};
use crate::delegators::get_values_from_node;

// The query parameters
#[derive(Debug, Deserialize)]
pub struct ListOptions {
    pub offset: Option<usize>,
    pub limit: Option<usize>,
}

#[derive(Clone)]
pub struct Delegator{
    pub conn: Quaint
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct EntityData{
    pub entity: String,
    pub values: serde_json::Value,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct ServiceResult<T>{
    pub data: T,
}

fn table_name(ent: &str) -> String {
    ent.to_snake_case()
}

impl Delegator{
    pub async fn new() -> crate::Result<Self> {
        // let url = "mysql://root:root@localhost:3306/ofbiz";
        let url = "postgres://ofbiz:ofbiz@localhost:5432/ofbiz";
        // dotenv::dotenv().ok();
        // let url = env::var("OFBIZ_URL").unwrap();
        Ok(Delegator { conn: (Quaint::new(url).await?) })
    }

    pub async fn find<'a, T>(&self, entity_name: &str, conditions: T) -> crate::Result<GenericValues>
    where
        T: Into<ConditionTree<'a>>,{
        let query = Select::from_table(entity_name.to_snake_case()).so_that(conditions);
        let result = self.conn.select(query).await?;
        Ok(GenericValues{ entity_name: entity_name.to_string(), rs: result,
            include_null_fields: false, include_internal_fields: false })
    }

    pub async fn find_all(&self, entity_name: &str, include_null_fields:bool,
                          include_internal_fields:bool) -> crate::Result<GenericValues> {
        let query = Select::from_table(entity_name.to_snake_case());
        let result = self.conn.select(query).await?;
        Ok(GenericValues{ entity_name: entity_name.to_string(), rs: result,
            include_null_fields: include_null_fields,
            include_internal_fields: include_internal_fields })
    }

    pub async fn wrap_result<T>(&self, result: ResultSet) -> crate::Result<Vec<T>>
    where T: DeserializeOwned, {
        let jval = serde_json::Value::from(result);
        let rows = jval.as_array();
        debug!("total {}", rows.unwrap().len());

        let mut items = Vec::new();
        for row in rows.unwrap() {
            let v = serde_json::from_value::<T>(row.to_owned())?;
            items.push(v);
        }
        Ok(items)
    }

    pub async fn list<T>(&self, entity_name: &str) -> crate::Result<Vec<T>>
    where T: DeserializeOwned, {
        let query = Select::from_table(entity_name.to_snake_case());
        let result = self.conn.select(query).await?;
        let r=self.wrap_result::<T>(result).await?;
        Ok(r)
    }

    pub async fn list_with_options<T>(&self, entity_name: &str, options:ListOptions) -> crate::Result<Vec<T>>
    where T: DeserializeOwned, {
        let query = Select::from_table(entity_name.to_snake_case())
            .limit(options.limit.unwrap_or(100)).offset(options.offset.unwrap_or(0));
        let result = self.conn.select(query).await?;
        let r=self.wrap_result::<T>(result).await?;
        Ok(r)
    }

    pub async fn list_for<T>(&self, entity_name: &str, conditions: ConditionTree<'_>) -> crate::Result<Vec<T>>
    where T: DeserializeOwned, {
        let query = Select::from_table(entity_name.to_snake_case()).so_that(conditions);
        let result = self.conn.select(query).await?;
        let r=self.wrap_result::<T>(result).await?;
        Ok(r)
    }

    pub async fn store(&self, ent_name: &str, values: &serde_json::Map<String,serde_json::Value>) -> crate::Result<u64> {
        let (cols,vals)=get_values_from_map(values)?;
        self.inner_store(ent_name, cols, vals).await
    }

    pub async fn store_string_map(&self, ent_name: &str, values: HashMap<String,String>) -> crate::Result<u64> {
        let (cols,vals)=get_values_from_string_map(ent_name, values)?;
        self.inner_store(ent_name, cols, vals).await
    }

    pub async fn inner_store<'a>(&self, ent_name: &str, cols:Vec<String>, vals:Vec<quaint::Value<'a>>) -> crate::Result<u64> {
        let table=ent_name.to_snake_case();
        debug!("cols -> {:?}", cols);
        debug!("vals -> {:?}", vals);
        let insert: Insert<'_> = Insert::multi_into(table, cols)
            .values(vals).into();
        let changes = self.conn.execute(
            insert.on_conflict(OnConflict::DoNothing).into()).await?;

        Ok(changes)
    }

    pub async fn store_entity(&self, ppd: &EntityData) -> crate::Result<ServiceResult<u64>> {
        debug!("store entity {} ->", ppd.entity);

        if let Some(values)=ppd.values.as_object() {
            let map_vals: HashMap<String, String> = values.into_iter()
                .map(|(k, v)| (k.to_owned(), v.as_str().unwrap().to_string()))
                .collect();
            debug!("{:?}", map_vals);
            let changes = self.store_string_map(ppd.entity.as_str(), map_vals).await?;
            Ok(ServiceResult{data:changes})
        }else{
            Err(crate::ServiceError::DataFormatError{
                info: format!("Cannot extract data for entity {}", ppd.entity),
            })
        }
    }
}

pub struct GenericValues{
    pub entity_name: String,
    pub rs: ResultSet,
    include_null_fields: bool,
    include_internal_fields: bool,
}

impl From<GenericValues> for serde_json::Value {
    fn from(rs: GenericValues) -> Self {
        use serde_json::Map;

        let internal_fields=vec!["created_stamp", "created_tx_stamp",
                                 "last_updated_stamp", "last_updated_tx_stamp"];
        let result_set=rs.rs;
        let columns: Vec<String> = result_set.columns().iter().map(ToString::to_string).collect();
        let mut result = Vec::new();

        for row in result_set.into_iter() {
            let mut object = Map::new();

            for (idx, p_value) in row.into_iter().enumerate() {
                let column_name = &columns[idx];
                let val=serde_json::Value::from(p_value);
                if let serde_json::Value::Null=val{
                    continue;
                }
                if internal_fields.contains(&column_name.as_str()) {
                    continue;
                }
                object.insert(column_name.to_camel_case(), val);
            }

            result.push(serde_json::Value::Object(object));
        }

        serde_json::Value::Array(result)
    }
}


pub async fn result_str(rs: GenericValues) -> String {
    let jval=serde_json::Value::from(rs);
    serde_json::to_string_pretty(&jval).expect("pretty json")
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use serde_json::{json, to_string_pretty};
    use chrono::{DateTime, Utc};

    // source from: $ cargo run --bin seed gen UserLogin dto_orig
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
        let rs=delegator.find_all("UserLogin", true, true).await?;
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
        let result=delegator.find("OrderItem", conditions).await?;

        let cols = result.rs.columns();
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

    #[tokio::test]
    async fn store_entity_data() -> crate::Result<()> {
        let delegator = Delegator::new().await?;

        let ppd:EntityData=serde_json::from_value(json!({
                "entity": "SecurityGroupPermission",
                "values": {
                  "groupId": "VIEWADMIN",
                  "permissionId": "PAY_INFO_VIEW",
                  "fromDate": "2001-05-13 12:00:00.0"
                }
            }))?;
        println!("{} ->", ppd.entity);

        if let Some(values)=ppd.values.as_object() {
            let map_vals: HashMap<String, String> = values.into_iter()
                .map(|(k, v)| (k.to_owned(), v.as_str().unwrap().to_string()))
                .collect();
            println!("{:?}", map_vals);
            let changes = delegator.store_string_map(ppd.entity.as_str(), map_vals).await;
            if let Err(ref errors) = changes {
                // print_errs(errors);
                println!("{:?}", errors);
            }
            println!("changes: {:?}", changes);
            // assert_eq!(1, changes.unwrap());
        }
        Ok(())
    }
}


