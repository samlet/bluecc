use crate::delegators::Delegator;
use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};
use seed::GenericError;
use inflector::Inflector;
use std::collections::HashMap;
use serde::Serialize;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person{
    // keys
    pub party_id: Option<String>,
    // fields
    pub salutation: Option<String>,
    pub first_name: Option<String>,
    pub middle_name: Option<String>,
    pub last_name: Option<String>,
    pub personal_title: Option<String>,
    pub suffix: Option<String>,
    pub nickname: Option<String>,
    pub first_name_local: Option<String>,
    pub middle_name_local: Option<String>,
    pub last_name_local: Option<String>,
    pub other_local: Option<String>,
    pub member_id: Option<String>,
    pub gender: Option<String>,
    pub birth_date: Option<chrono::NaiveDate>,
    pub deceased_date: Option<chrono::NaiveDate>,
    pub height: Option<bigdecimal::BigDecimal>,
    pub weight: Option<bigdecimal::BigDecimal>,
    pub mothers_maiden_name: Option<String>,
    pub old_marital_status: Option<String>,
    pub marital_status_enum_id: Option<String>,
    pub social_security_number: Option<String>,
    pub passport_number: Option<String>,
    pub passport_expire_date: Option<chrono::NaiveDate>,
    pub total_years_work_experience: Option<bigdecimal::BigDecimal>,
    pub comments: Option<String>,
    pub employment_status_enum_id: Option<String>,
    pub residence_status_enum_id: Option<String>,
    pub occupation: Option<String>,
    pub years_with_employer: Option<i64>,
    pub months_with_employer: Option<i64>,
    pub existing_customer: Option<String>,
    pub card_id: Option<String>
}

fn pretty<T>(val:&T) -> String
where
    T: ?Sized + Serialize,{
    serde_json::to_string_pretty(val).unwrap()
}

async fn print_person(party_id: &str, items: &Vec<Person>) -> anyhow::Result<()> {
    let ex_sts:Vec<&Person>=items.iter()
        .filter(|&n|n.party_id==Some(party_id.to_string()))
        .collect();
    for ex in &ex_sts{
        println!("{} => ", ex.last_name.as_ref().unwrap());
        println!("{}", pretty(ex));
    }
    Ok(())
}

#[tokio::test]
async fn list_ent_works() -> Result<(), GenericError> {
    let delegator=Delegator::new().await?;
    let rs:Vec<Person>=delegator.list("Person").await?;
    println!("total {}", rs.len());
    print_person("SCRUMADMIN", &rs).await?;
    Ok(())
}

