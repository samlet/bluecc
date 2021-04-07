use chrono::{DateTime, Utc};

pub trait Role : Send + Sync{
    fn get_user_login_id(&self) -> String;
    fn get_group_id(&self) -> String;
}

pub trait Permission : Send + Sync{
    fn get_group_id(&self) -> String;
    fn get_permission_id(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
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


#[derive(Debug, Deserialize, Serialize, Clone)]
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

impl Role for UserLoginSecurityGroup{
    fn get_user_login_id(&self) -> String {
        self.user_login_id.as_ref().expect("login id").to_string()
    }

    fn get_group_id(&self) -> String {
        self.group_id.as_ref().expect("group id").to_string()
    }
}

impl Permission for SecurityGroupPermission{
    fn get_group_id(&self) -> String {
        self.group_id.as_ref().expect("group id").to_string()
    }

    fn get_permission_id(&self) -> String {
        self.permission_id.as_ref().expect("permission id").to_string()
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    use crate::delegators::Delegator;

    #[tokio::test]
    async fn list_grps_works() -> crate::Result<()> {
        let delegator=Delegator::new().await?;
        let rs:Vec<UserLoginSecurityGroup>=delegator.list("UserLoginSecurityGroup").await?;
        println!("total {}", rs.len());
        rs.iter().for_each(|r|
            println!("{}", serde_json::to_string_pretty(r).unwrap()));
        Ok(())
    }

    #[tokio::test]
    async fn list_perms_works() -> crate::Result<()> {
        let delegator=Delegator::new().await?;
        let rs:Vec<SecurityGroupPermission>=delegator.list("SecurityGroupPermission").await?;
        println!("total {}", rs.len());
        rs.iter().for_each(|r|
            println!("{}", serde_json::to_string_pretty(r).unwrap()));
        Ok(())
    }
}

