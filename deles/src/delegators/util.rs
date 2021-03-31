use chrono::{NaiveDateTime, Datelike};
use serde::{Serialize, de};
use std::collections::HashMap;
use itertools::Itertools;
use serde::de::DeserializeOwned;
use crate::GenericError;
use thiserror::private::DisplayAsDisplay;

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
        .set_content_arrangement(comfy_table::ContentArrangement::Dynamic)
        .set_table_width(140);

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

#[cfg(test)]
mod lib_tests {
    use super::*;
    use crate::delegators::status_procs::StatusItemRaw;
    use crate::delegators::Delegator;

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
}


