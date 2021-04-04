use quaint::{prelude::*, ast::*, single::Quaint, visitor::{Visitor, Postgres},
                 connector::{Queryable, TransactionCapable},
    };
use inflector::Inflector;
use std::collections::HashMap;
use itertools::Itertools;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct PrimaryFinder{
    pub entity_name: String,
    pub values: Vec<String>,
}

impl PrimaryFinder{
    pub fn new(ent:&str, values:&Vec<&str>) -> Self{
        PrimaryFinder{ entity_name: ent.to_string(),
            values: values.iter().map(|s|s.to_string()).collect_vec() }
    }
    pub fn build_conditions(&self) -> crate::Result<ConditionTree> {
        let ent=seed::get_entity_model(self.entity_name.as_str())?;
        let mut conditions:ConditionTree =ConditionTree::NoCondition;
        for (i,p) in ent.primary_keys.iter().enumerate(){
            let fld_name=p.field_name.to_snake_case();
            if i==0{
                conditions=fld_name.equals(self.values[i].as_str()).into();
            }else {
                conditions = conditions.and(fld_name.equals(self.values[i].as_str())).into();
            }
        }

        Ok(conditions)
    }
    pub fn selector(&self) -> crate::Result<Select<'_>>{
        let conditions=self.build_conditions()?;
        // select expr
        let query = Select::from_table(self.entity_name.to_snake_case()).so_that(conditions);
        Ok(query)
    }

    pub fn deleter(&self) -> crate::Result<Delete<'_>>{
        let conditions=self.build_conditions()?;
        let query = Delete::from_table(self.entity_name.to_snake_case()).so_that(conditions);
        Ok(query)
    }

    pub fn updater(&self, paras: HashMap<String, String>) -> crate::Result<Update<'_>>{
        let conditions=self.build_conditions()?;
        let mut query=Update::table(self.entity_name.to_snake_case());
        for (k,v) in paras{
            query=query.set(k,v)
        }
        query=query.so_that(conditions);
        Ok(query)
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn selector_works() -> anyhow::Result<()> {
        let finder=PrimaryFinder::new("SecurityGroupPermission",
             &vec!["admin","read","2004-03-04 18:48:34.612"]);
        let query=finder.selector()?;
        let (sql, params) = Postgres::build(query)?;
        println!("{} ==> {:?}", sql, params);
        Ok(())
    }
}
