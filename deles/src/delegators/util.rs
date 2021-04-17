use chrono::{NaiveDateTime, Datelike};
use serde::{Serialize, de};
use std::collections::HashMap;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use crate::GenericError;
use thiserror::private::DisplayAsDisplay;
use crate::delegators::Delegator;
use inflector::Inflector;

pub fn pretty<T>(val:&T) -> String
where
    T: ?Sized + Serialize,{
    serde_json::to_string_pretty(val).unwrap()
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynamicValue {
    #[serde(flatten)]
    pub values: HashMap<String, serde_json::Value>,
}

pub fn render<'a, T>(values: &'a Vec<T>) -> Result<(), GenericError>
where
    T: Serialize+de::Deserialize<'a>,{
    let json_str = serde_json::to_string(values)?;
    let dyn_vals:Vec<DynamicValue>=serde_json::from_str(json_str.as_str())?;
    render_table(&dyn_vals);
    Ok(())
}

pub fn render_table(values: &Vec<DynamicValue>) {
    use comfy_table::presets::UTF8_FULL;
    use comfy_table::modifiers::UTF8_ROUND_CORNERS;

    if values.is_empty(){
        return;
    }

    let mut table = comfy_table::Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
        // .set_table_width(140);

    let cols=values.first().unwrap().values.keys().collect_vec();
    table.set_header(&cols);

    for r in values.iter() {
        let mut row=Vec::new();
        let rec=&r.values;
        for &col in &cols{
            let val= rec.get(col).unwrap();
            let sval:String=if val.is_string(){val.as_str().unwrap().to_string()} else{ val.to_string()};
            row.push(sval);
        }
        table.add_row(row);
    }
    println!("{}", table);
}

pub async fn browse_data(delegator:&Delegator, ent:&str, cols: &Vec<&str>) -> crate::Result<()> {
    use comfy_table::presets::UTF8_FULL;
    use comfy_table::modifiers::UTF8_ROUND_CORNERS;
    use quaint::{prelude::*, ast::*, single::Quaint,
                 connector::{Queryable, TransactionCapable},
    };

    let table = ent.to_snake_case();
    let query = Select::from_table(table).columns(cols);
    let result = delegator.conn.select(query).await?;

    let mut table = comfy_table::Table::new();
    table
        .load_preset(UTF8_FULL)
        .apply_modifier(UTF8_ROUND_CORNERS)
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
    table.set_header(cols);
    let columns: Vec<String> = result.columns().iter().map(ToString::to_string).collect();
    for row in result.into_iter() {
        let mut table_row = Vec::new();
        for (idx, p_value) in row.into_iter().enumerate() {
            let _column_name = &columns[idx];
            let val = serde_json::Value::from(p_value);
            if let serde_json::Value::Null = val {
                table_row.push("".to_string());
            } else {
                table_row.push(if val.is_string() {
                    val.as_str().unwrap().to_string()
                } else { val.to_string() });
            }
        }

        table.add_row(table_row);
    }

    println!("{}", table);

    Ok(())
}

/*
pub fn print_errs(errors:&crate::Error){
    eprintln!("Error level - description");
    errors
        .iter()
        .enumerate()
        .for_each(|(index, error)| eprintln!("└> {} - {}", index, error));

    if let Some(backtrace) = errors.backtrace() {
        eprintln!("{:?}", backtrace);
    }
}
*/

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::delegators::status_procs::StatusItemRaw;
    use crate::delegators::Delegator;
    use quaint::{prelude::*, ast::*, single::Quaint, visitor::{Visitor, Postgres},
                 connector::{Queryable, TransactionCapable},
    };
    use inflector::Inflector;

    #[test]
    fn dt_works() -> anyhow::Result<()> {
        let created_date =
            NaiveDateTime::parse_from_str("2004-03-04 18:48:34.612",
                                          "%Y-%m-%d %H:%M:%S%.f")?;

        println!("{} - {:?}", created_date.year(), created_date);
        Ok(())
    }

    #[tokio::test]
    async fn render_table_works() -> anyhow::Result<()> {
        let delegator=Delegator::new().await?;
        let rs:Vec<StatusItemRaw>=delegator.list("StatusItem").await?;
        println!("total {}", rs.len());
        render(&rs)?;

        Ok(())
    }

    #[tokio::test]
    async fn browse_rec_works() -> crate::Result<()> {
        use comfy_table::presets::UTF8_FULL;
        use comfy_table::modifiers::UTF8_ROUND_CORNERS;

        let ent="ProductType";
        let table=ent.to_snake_case();

        let cols=vec!["product_type_id", "description"];
        let delegator=Delegator::new().await?;
        let query = Select::from_table(table).columns(&cols);
        let result = delegator.conn.select(query).await?;

        let mut table = comfy_table::Table::new();
        table
            .load_preset(UTF8_FULL)
            .apply_modifier(UTF8_ROUND_CORNERS)
            .set_content_arrangement(comfy_table::ContentArrangement::Dynamic);
        table.set_header(&cols);
        let columns: Vec<String> = result.columns().iter().map(ToString::to_string).collect();
        for row in result.into_iter() {
            let mut table_row=Vec::new();
            for (idx, p_value) in row.into_iter().enumerate() {
                let _column_name = &columns[idx];
                let val = serde_json::Value::from(p_value);
                if let serde_json::Value::Null = val {
                    table_row.push("".to_string());
                }else {
                    table_row.push(if val.is_string(){
                        val.as_str().unwrap().to_string()
                    } else {val.to_string()});
                }
            }

            table.add_row(table_row);
        }

        println!("{}", table);

        Ok(())
    }

    #[tokio::test]
    async fn browse_works() -> crate::Result<()> {
        let ent="ProductType";
        let cols=vec!["product_type_id", "description"];
        let delegator=Delegator::new().await?;
        browse_data(&delegator, ent, &cols).await?;

        Ok(())
    }

    #[test]
    fn conditions_works() -> anyhow::Result<()> {
        let query = Select::from_table("users").so_that("foo".like("bar"));
        let (sql, params) = Postgres::build(query)?;
        println!("{} ==> {:?}", sql, params);

        let mut conditions:ConditionTree = "product_id".equals("WG-1111").into();
        conditions=conditions.and("unit_price".less_than(100.00)).into();
        conditions=conditions.and("unit_price".greater_than(10.00)).into();
        let query = Select::from_table("users").so_that(conditions);
        let (sql, params) = Postgres::build(query)?;
        println!("{} ==> {:?}", sql, params);

        Ok(())
    }

    #[test]
    fn primary_keys_works() -> anyhow::Result<()> {
        let pk_vals=vec!["WG-1111"];
        let ent=seed::get_entity_model("Product")?;
        let mut conditions:ConditionTree =ConditionTree::NoCondition;
        for (i,p) in ent.primary_keys.iter().enumerate(){
            if i==0{
                conditions=p.field_name.to_owned().equals(pk_vals[i]).into();
            }else {
                conditions = conditions.and(p.field_name.to_owned().equals(pk_vals[i])).into();
            }
        }

        let query = Select::from_table("product").so_that(conditions);
        let (sql, params) = Postgres::build(query)?;
        println!("{} ==> {:?}", sql, params);

        Ok(())
    }

    #[test]
    fn multi_primary_keys_works() -> anyhow::Result<()> {
        let pk_vals=vec!["admin","read","2004-03-04 18:48:34.612"];
        let ent_name="SecurityGroupPermission";
        
        let ent=seed::get_entity_model(ent_name)?;
        let mut conditions:ConditionTree =ConditionTree::NoCondition;
        for (i,p) in ent.primary_keys.iter().enumerate(){
            let fld_name=p.field_name.to_snake_case();
            if i==0{
                conditions=fld_name.equals(pk_vals[i]).into();
            }else {
                conditions = conditions.and(fld_name.equals(pk_vals[i])).into();
            }
        }

        // select expr
        let query = Select::from_table(ent_name.to_snake_case()).so_that(conditions.clone());
        let (sql, params) = Postgres::build(query)?;
        println!("{} ==> {:?}", sql, params);

        // delete expr
        let query = Delete::from_table(ent_name.to_snake_case()).so_that(conditions.clone());
        let (sql, params) = Postgres::build(query)?;
        println!("{} ==> {:?}", sql, params);

        // update expr
        let mut query=Update::table(ent_name.to_snake_case());
        let paras: HashMap<&str, i32> =
            [("Norway", 100),
             ("Denmark", 50),
             ("Iceland", 10)]
             .iter().cloned().collect();
        for (k,v) in paras{
            query=query.set(k,v)
        }
        query=query.so_that(conditions.clone());
        let (sql, params) = Postgres::build(query)?;
        println!("{} ==> {:?}", sql, params);

        Ok(())
    }

    #[derive(Serialize, Deserialize)]
    enum ConditionExpr{
        Equals{fld:String, val:String},
        Between{fld:String, start:String, end:String},
    }

    #[derive(Serialize, Deserialize)]
    struct Exprs{
        pub conditions: Vec<ConditionExpr>
    }

    #[test]
    fn condition_json_works() -> anyhow::Result<()> {
        let range_val=ConditionExpr::Between {
            fld: "age".to_string(),
            start: "1".to_string(),
            end: "11".to_string()
        };
        let exprs=Exprs{ conditions: vec![range_val] };
        println!("{}", pretty(&exprs));
        Ok(())
    }
}


