use crate::delegators::Delegator;
use quaint::{prelude::*, ast::*, single::Quaint,
             connector::{Queryable, TransactionCapable},
};
use seed::GenericError;
use inflector::Inflector;
use std::collections::HashMap;
use serde::Serialize;
use chrono::{DateTime, Utc};

// $ cargo run --bin seed gen Person dto_orig
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Person{
    // keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub salutation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub personal_title: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub suffix: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub nickname: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub first_name_local: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub middle_name_local: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_name_local: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub other_local: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub member_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gender: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub birth_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub deceased_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub height: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub weight: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mothers_maiden_name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub old_marital_status: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub marital_status_enum_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub social_security_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_number: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub passport_expire_date: Option<chrono::NaiveDate>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub total_years_work_experience: Option<bigdecimal::BigDecimal>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub comments: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub employment_status_enum_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub residence_status_enum_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub occupation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub years_with_employer: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub months_with_employer: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub existing_customer: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub card_id: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Party{
    // keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_id: Option<String>,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub party_type_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub preferred_currency_uom_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_by_user_login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_date: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_modified_by_user_login: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data_source_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_unread: Option<String>
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

#[tokio::test]
async fn list_parties_works() -> Result<(), GenericError> {
    let delegator=Delegator::new().await?;
    let conditions = "party_type_id".equals("PARTY_GROUP");
    let rs:Vec<Party>=delegator.list_for("Party", conditions.into()).await?;
    println!("total {}", rs.len());
    for r in &rs{
        println!("{}", pretty(r));
    }
    Ok(())
}

#[test]
fn seed_toml_works() -> anyhow::Result<()> {
    let seed_rec=r#"
        party_id = "TestParty"
        first_name = "Test"
        last_name = "Party"
    "#;
    let person:Person=toml::from_str(seed_rec)?;
    println!("{}", pretty(&person));
    Ok(())
}

