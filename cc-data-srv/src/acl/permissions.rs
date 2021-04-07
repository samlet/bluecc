use casbin::prelude::*;
use crate::acl::Result;
use deles::delegators::Delegator;
use deles::resources::policy::{UserLoginSecurityGroup, SecurityGroupPermission};
use crate::acl::{role_lines, perm_lines};
use crate::acl::adapters::SecurityAdapter;
use std::sync::Arc;

type SharedEnforcer = Arc<Enforcer>;

#[derive(Clone)]
pub struct SecurityManager{
    delegator: Delegator,
    e: SharedEnforcer,
}

impl SecurityManager{
    pub async fn new(delegator: Delegator) -> Result<Self> {
        let e=SecurityManager::load_perms(&delegator).await?;
        Ok(SecurityManager { delegator: delegator, e: Arc::new(e)})
    }

    async fn load_perms(delegator: &Delegator) -> Result<Enforcer>{
        let rs:Vec<SecurityGroupPermission>=delegator.list("SecurityGroupPermission").await?;
        let p_lines=perm_lines(&rs);
        let rs:Vec<UserLoginSecurityGroup>=delegator.list("UserLoginSecurityGroup").await?;
        let r_lines=role_lines(&rs);

        let source=common::prelude::cat(&p_lines, &r_lines);
        let m1 = DefaultModel::from_str(include_str!("rbac_model.conf"))
            .await.unwrap();
        let adapter = SecurityAdapter::new(source);
        let e=Enforcer::new(m1, adapter).await?;
        Ok(e)
    }

    pub fn enforcer(&self) -> &Enforcer{
        &self.e
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use casbin::rhai::ImmutableString;
    use casbin::function_map::key_match;

    #[tokio::test]
    async fn test_file() -> Result<()> {
        let mut e = Enforcer::new("examples/rbac_with_domains_model.conf",
                                  "examples/rbac_with_domains_policy.csv").await?;
        e.enable_log(true);
        assert_eq!(true, e.enforce(("alice", "domain1", "data1", "read"))?);

        // ...
        let filter = Filter {
            p: vec!["", "domain1"],
            g: vec!["", "", "domain1"],
        };

        e.load_filtered_policy(filter).await.unwrap();
        assert!(e.enforce(("alice", "domain1", "data1", "read")).unwrap());
        assert!(e.enforce(("alice", "domain1", "data1", "write")).unwrap());
        assert!(!e.enforce(("alice", "domain1", "data2", "read")).unwrap());
        assert!(!e.enforce(("alice", "domain1", "data2", "write")).unwrap());
        assert!(!e.enforce(("bob", "domain2", "data2", "read")).unwrap());
        assert!(!e.enforce(("bob", "domain2", "data2", "write")).unwrap());

        Ok(())
    }

    /// ref: https://casbin.org/docs/en/rbac-api
    #[tokio::test]
    async fn roles_works() -> Result<()> {
        let mut e = Enforcer::new("examples/rbac_with_domains_model.conf",
                                  "examples/rbac_with_domains_policy.csv").await?;
        e.enable_log(true);

        let roles = e.get_roles_for_user("alice", Some("domain1"));
        println!("{:?}", roles);
        assert_eq!(vec!["admin"], roles);

        let users = e.get_users_for_role("admin", Some("domain1"));
        assert_eq!(vec!["alice"], users);

        let has = e.has_role_for_user("alice", "admin", Some("domain1"));
        assert!(has);

        let roles = vec!["data1_admin".to_owned(), "data2_admin".to_owned()];
        let all_added = e.add_roles_for_user("alice", roles, Some("domain1")).await?;
        assert!(all_added);
        assert!(e.has_role_for_user("alice", "data1_admin", Some("domain1")));

        Ok(())
    }

    #[tokio::test]
    async fn permission_works() -> Result<()> {
        let mut e = Enforcer::new("examples/rbac_with_domains_model.conf",
                                  "examples/rbac_with_domains_policy.csv").await?;
        e.enable_log(true);
        let d=Some("domain1");
        let roles=e.get_implicit_roles_for_user("alice", d);
        println!("{:?}", roles);

        let users=e.get_implicit_permissions_for_user("alice", d);
        println!("{:?}", users);

        let permissions = vec![
            vec!["data1".to_owned(), "read".to_owned()],
            vec!["data2".to_owned(), "write".to_owned()],
        ];

        let _all_added = e.add_permissions_for_user("alice", permissions).await?;
        let has = e.has_permission_for_user("alice",
                                            vec!["data1".to_owned(), "read".to_owned()]);
        println!("has data1 read: {}", has);
        Ok(())
    }

    #[tokio::test]
    async fn cust_fn_works() -> Result<()> {

        let m1 = DefaultModel::from_file("examples/keymatch_custom_model.conf")
            .await
            .unwrap();
        let adapter1 = FileAdapter::new("examples/keymatch_policy.csv");
        let mut e = Enforcer::new(m1, adapter1).await.unwrap();

        e.add_function(
            "keyMatchCustom",
            |s1: ImmutableString, s2: ImmutableString| key_match(&s1, &s2),
        );

        assert_eq!(true, e.enforce(("alice", "/alice_data/123", "GET")).unwrap());
        assert_eq!(true, e.enforce(("alice", "/alice_data/resource1", "POST")).unwrap());

        assert_eq!(true, e.enforce(("bob", "/alice_data/resource2", "GET")).unwrap());

        assert_eq!(true, e.enforce(("bob", "/bob_data/resource1", "POST")).unwrap());

        assert_eq!(true, e.enforce(("cathy", "/cathy_data", "GET")).unwrap());
        assert_eq!(true, e.enforce(("cathy", "/cathy_data", "POST")).unwrap());
        Ok(())
    }

    #[tokio::test]
    async fn security_manager_works() -> Result<()> {
        let delegator=Delegator::new().await?;
        let secmgr=SecurityManager::new(delegator).await?;
        let e=secmgr.enforcer();

        assert_eq!(true, e.enforce(("blog_editor", "userpref", "admin")).unwrap());
        assert_eq!(false, e.enforce(("DemoLeadOwner", "userpref", "admin")).unwrap());

        Ok(())
    }
}

