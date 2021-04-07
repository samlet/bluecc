use lazy_static::lazy_static;
use regex::Regex;

use std::borrow::Cow;
use chrono::{DateTime, Utc};
use serde_json::json;
use common::prelude::utc_fmt;
use itertools::Itertools;

macro_rules! regex {
    ($re:expr) => {
        ::regex::Regex::new($re).unwrap()
    };
}

lazy_static! {
    static ref ESC_A: Regex = regex!(r"\b(r\d*|p\d*)\.");
    static ref ESC_G: Regex = regex!(
        r"\b(g\d*)\(((?:\s*[r|p]\d*\.\w+\s*,\s*){1,2}\s*[r|p]\d*\.\w+\s*)\)"
    );
    static ref ESC_C: Regex = regex!(r#"(\s*"[^"]*"?|\s*[^,]*)"#);
    pub(crate) static ref ESC_E: Regex = regex!(r"\beval\(([^)]*)\)");
}

pub fn parse_csv_line<S: AsRef<str>>(line: S) -> Option<Vec<String>> {
    let line = line.as_ref().trim();
    if line.is_empty() || line.starts_with('#') {
        return None;
    }

    let mut res = vec![];
    for col in ESC_C.find_iter(line).map(|m| m.as_str().trim()) {
        res.push({
            if col.len() >= 2 && col.starts_with('"') && col.ends_with('"') {
                col[1..col.len() - 1].to_owned()
            } else {
                col.to_owned()
            }
        })
    }
    if res.is_empty() {
        None
    } else {
        Some(res)
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct SecurityGroupPermission{
    // keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub permission_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<DateTime<Utc>>,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thru_date: Option<DateTime<Utc>>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct UserLogin{
    // keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_login_id: Option<String>,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub current_password: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub password_hint: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub is_system: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub enabled: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub has_logged_out: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub require_password_change: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_currency_uom: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_locale: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_time_zone: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_date_time: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub successive_failed_logins: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub external_auth_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_ldap_dn: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub disabled_by: Option<String>
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all(serialize = "camelCase", deserialize = "camelCase"))]
pub struct UserLoginSecurityGroup{
    // keys
    #[serde(skip_serializing_if = "Option::is_none")]
    pub user_login_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub group_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub from_date: Option<DateTime<Utc>>,
    // fields
    #[serde(skip_serializing_if = "Option::is_none")]
    pub thru_date: Option<DateTime<Utc>>
}

fn perm_lines(recs: &Vec<SecurityGroupPermission>) -> Vec<String>{
    let empty="".to_string();
    let lines=recs.iter()
        .map(|f|vec!["p".to_string(),
                     f.group_id.as_ref().unwrap_or(&empty).to_lowercase(),
                     perm_parts(f.permission_id.as_ref())])
        .map(|r|r.join(", "))
        .collect::<Vec<String>>();
    lines
}

fn perm_parts(perm_str: Option<&String>) -> String{
    perm_str.unwrap().to_lowercase().split("_").join(", ")
}

fn role_lines(rs: &Vec<UserLoginSecurityGroup>) -> Vec<String>{
    let empty="".to_string();
    let lines=rs.iter()
        .map(|f|vec!["g".to_string(),
                    f.user_login_id.as_ref().unwrap_or(&empty).to_owned(),
                    f.group_id.as_ref().unwrap_or(&empty).to_lowercase(), ])
        .map(|r|r.join(", "))
        .collect::<Vec<String>>();
    lines
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use itertools::Itertools;

    #[test]
    fn parse_csv_works() -> anyhow::Result<()> {
        assert_eq!(
            parse_csv_line("alice, domain1, data1, action1"),
            Some(vec![
                "alice".to_owned(),
                "domain1".to_owned(),
                "data1".to_owned(),
                "action1".to_owned()
            ])
        );

        Ok(())
    }

    #[test]
    fn test_csv_parse_5() {
        assert_eq!(
            parse_csv_line(
                "alice, \"domain1, domain2\", \"data1, data2\", action1"
            ),
            Some(vec![
                "alice".to_owned(),
                "domain1, domain2".to_owned(),
                "data1, data2".to_owned(),
                "action1".to_owned()
            ])
        )
    }

    #[test]
    fn test_csv_parse_10() {
        assert_eq!(
            parse_csv_line("r.sub.Status == \"ACTIVE\", /data1, read"),
            Some(vec![
                "r.sub.Status == \"ACTIVE\"".to_owned(),
                "/data1".to_owned(),
                "read".to_owned()
            ])
        );
    }

    #[test]
    fn convert_security_works() -> anyhow::Result<()> {
        // let mut policies = String::new();
        // policies.push_str(&format!("{}, {}\n", ptype, rule.join(",")));

        let r1:SecurityGroupPermission= serde_json::from_value(json!({
              "fromDate": utc_fmt("2001-05-13 12:00:00.0")?,
              "groupId": "FULLADMIN",
              "permissionId": "MARKETING_ADMIN"
          }))?;
        let r2:SecurityGroupPermission= serde_json::from_value(json!({
              "permissionId": "MARKETING_CREATE",
              "fromDate": utc_fmt("2001-05-13 12:00:00.0")?,
              "groupId": "FLEXADMIN"
          }))?;

        let recs=vec![r1, r2];

        println!("{:?}", recs);

        let lines=perm_lines(&recs);
        for l in lines{
            println!("{}", l);
        }

        Ok(())
    }

    #[test]
    fn convert_sec_grp_works() -> anyhow::Result<()> {
        let rs:Vec<UserLoginSecurityGroup>= serde_json::from_value(json!([
            {
              "groupId": "VIEWADMIN",
              "userLoginId": "DemoLeadOwner",
              "fromDate": utc_fmt("2001-05-13 00:00:00.000")?
            },
            {
              "fromDate": utc_fmt("2001-05-13 00:00:00.000")?,
              "groupId": "VIEWADMIN",
              "userLoginId": "DemoLeadOwner1"
            }
        ]))?;

        println!("{:?}", rs);

        let lines=role_lines(&rs);
        for l in lines{
            println!("{}", l);
        }

        Ok(())
    }

}

