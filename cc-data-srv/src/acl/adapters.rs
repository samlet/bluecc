use std::{
    io::{Error as IoError, ErrorKind},
    path::Path,
};

use tokio::{
    fs::File,
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
};

use async_trait::async_trait;

use std::convert::AsRef;
use casbin::{Model, Filter, Result, Adapter};
use casbin::error::ModelError;
use crate::acl::data_format::parse_csv_line;

pub struct SecurityAdapter {
    content: Vec<String>,
    is_filtered: bool,
}

type LoadPolicyFileHandler = fn(String, &mut dyn Model);
type LoadFilteredPolicyFileHandler<'a> =
    fn(String, &mut dyn Model, f: &Filter<'a>) -> bool;

impl SecurityAdapter
{
    pub fn new(p: Vec<String>) -> SecurityAdapter {
        SecurityAdapter {
            content: p,
            is_filtered: false,
        }
    }

    async fn load_policy_file(
        &self,
        m: &mut dyn Model,
        handler: LoadPolicyFileHandler,
    ) -> Result<()> {

        for line in &self.content {
            handler(line.clone(), m)
        }

        Ok(())
    }

    async fn load_filtered_policy_file<'a>(
        &self,
        m: &mut dyn Model,
        filter: Filter<'a>,
        handler: LoadFilteredPolicyFileHandler<'a>,
    ) -> Result<bool> {
        let mut is_filtered = false;
        for line in &self.content {
            if handler(line.clone(), m, &filter) {
                is_filtered = true;
            }
        }

        Ok(is_filtered)
    }

    async fn save_policy_file(&self, _text: String) -> Result<()> {
        // let mut file = File::create(&self.file_path).await?;
        // file.write_all(text.as_bytes()).await?;
        Ok(())
    }
}

#[async_trait]
impl Adapter for SecurityAdapter
{
    async fn load_policy(&self, m: &mut dyn Model) -> Result<()> {
        self.load_policy_file(m, load_policy_line).await?;
        Ok(())
    }

    async fn load_filtered_policy<'a>(
        &mut self,
        m: &mut dyn Model,
        f: Filter<'a>,
    ) -> Result<()> {
        self.is_filtered = self
            .load_filtered_policy_file(m, f, load_filtered_policy_line)
            .await?;

        Ok(())
    }

    async fn save_policy(&mut self, m: &mut dyn Model) -> Result<()> {
        // if self.file_path.as_ref().as_os_str().is_empty() {
        //     return Err(IoError::new(
        //         ErrorKind::Other,
        //         "save policy failed, file path is empty",
        //     )
        //     .into());
        // }

        let mut policies = String::new();
        let ast_map = m.get_model().get("p").ok_or_else(|| {
            ModelError::P("Missing policy definition in conf file".to_owned())
        })?;

        for (ptype, ast) in ast_map {
            for rule in ast.get_policy() {
                policies.push_str(&format!("{}, {}\n", ptype, rule.join(",")));
            }
        }

        if let Some(ast_map) = m.get_model().get("g") {
            for (ptype, ast) in ast_map {
                for rule in ast.get_policy() {
                    policies.push_str(&format!(
                        "{}, {}\n",
                        ptype,
                        rule.join(",")
                    ));
                }
            }
        }

        self.save_policy_file(policies).await?;
        Ok(())
    }

    async fn clear_policy(&mut self) -> Result<()> {
        self.save_policy_file(String::new()).await?;
        Ok(())
    }

    async fn add_policy(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rule: Vec<String>,
    ) -> Result<bool> {
        // this api shouldn't implement, just for convenience
        Ok(true)
    }

    async fn add_policies(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rules: Vec<Vec<String>>,
    ) -> Result<bool> {
        // this api shouldn't implement, just for convenience
        Ok(true)
    }

    async fn remove_policy(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rule: Vec<String>,
    ) -> Result<bool> {
        // this api shouldn't implement, just for convenience
        Ok(true)
    }

    async fn remove_policies(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _rule: Vec<Vec<String>>,
    ) -> Result<bool> {
        // this api shouldn't implement, just for convenience
        Ok(true)
    }

    async fn remove_filtered_policy(
        &mut self,
        _sec: &str,
        _ptype: &str,
        _field_index: usize,
        _field_values: Vec<String>,
    ) -> Result<bool> {
        // this api shouldn't implement, just for convenience
        Ok(true)
    }

    fn is_filtered(&self) -> bool {
        self.is_filtered
    }
}

fn load_policy_line(line: String, m: &mut dyn Model) {
    if line.is_empty() || line.starts_with('#') {
        return;
    }

    if let Some(tokens) = parse_csv_line(line) {
        let key = &tokens[0];

        if let Some(ref sec) = key.chars().next().map(|x| x.to_string()) {
            if let Some(ast_map) = m.get_mut_model().get_mut(sec) {
                if let Some(ast) = ast_map.get_mut(key) {
                    ast.get_mut_policy().insert(tokens[1..].to_vec());
                }
            }
        }
    }
}

fn load_filtered_policy_line<'a>(
    line: String,
    m: &mut dyn Model,
    f: &Filter<'a>,
) -> bool {
    if line.is_empty() || line.starts_with('#') {
        return false;
    }

    if let Some(tokens) = parse_csv_line(line) {
        let key = &tokens[0];

        let mut is_filtered = false;
        if let Some(ref sec) = key.chars().next().map(|x| x.to_string()) {
            if sec == "p" {
                for (i, rule) in f.p.iter().enumerate() {
                    if !rule.is_empty() && rule != &tokens[i + 1] {
                        is_filtered = true;
                    }
                }
            }
            if sec == "g" {
                for (i, rule) in f.g.iter().enumerate() {
                    if !rule.is_empty() && rule != &tokens[i + 1] {
                        is_filtered = true;
                    }
                }
            }
            if !is_filtered {
                if let Some(ast_map) = m.get_mut_model().get_mut(sec) {
                    if let Some(ast) = ast_map.get_mut(key) {
                        ast.get_mut_policy().insert(tokens[1..].to_vec());
                    }
                }
            }
        }

        is_filtered
    } else {
        false
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use casbin::{DefaultModel, Enforcer, CoreApi};
    use itertools::Itertools;
    use deles::delegators::Delegator;
    use deles::resources::policy::{UserLoginSecurityGroup, SecurityGroupPermission};
    use crate::acl::{role_lines, perm_lines};

    #[tokio::test]
    async fn adpater_works() -> Result<()> {
        let m1 = DefaultModel::from_file("examples/basic_model.conf")
            .await
            .unwrap();
        let lines=std::fs::read_to_string("examples/basic_policy.csv")?
            .split("\n").map(|l|l.to_string()).collect();

        let adapter1 = SecurityAdapter::new(lines);
        let mut e = Enforcer::new(m1, adapter1).await.unwrap();

        assert_eq!(true, e.enforce(("alice", "data1", "read")).unwrap());
        assert_eq!(false, e.enforce(("alice", "data1", "write")).unwrap());

        // let m2 = DefaultModel::from_file("examples/basic_model.conf")
        //     .await
        //     .unwrap();
        // let adapter2 = SecurityAdapter::new("examples/basic_inverse_policy.csv");
        // let _e2 = Enforcer::new(m2, adapter2).await.unwrap();

        // e.adapter = e2.adapter;
        // e.load_policy().await.unwrap();
        // assert_eq!(false, e.enforce(("alice", "data1", "read")).unwrap());
        // assert_eq!(true, e.enforce(("alice", "data1", "write")).unwrap());

        Ok(())
    }

    #[tokio::test]
    async fn load_policy_from_entity() -> anyhow::Result<()> {
        let delegator=Delegator::new().await?;
        let rs:Vec<SecurityGroupPermission>=delegator.list("SecurityGroupPermission").await?;
        println!("total perms {}", rs.len());
        let lines=perm_lines(&rs);
        for l in lines{
            println!("{}", l);
        }

        let rs:Vec<UserLoginSecurityGroup>=delegator.list("UserLoginSecurityGroup").await?;
        println!("total groups {}", rs.len());

        let lines=role_lines(&rs);
        for l in lines{
            println!("{}", l);
        }
        Ok(())
    }
}

