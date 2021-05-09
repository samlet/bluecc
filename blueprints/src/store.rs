use std::collections::HashMap;

use cdrs::authenticators::StaticPasswordAuthenticator;
use cdrs::cluster::session::{new as new_session, Session};
use cdrs::cluster::{ClusterTcpConfig, NodeTcpConfigBuilder, TcpConnectionPool};
use cdrs::load_balancing::RoundRobin;
use cdrs::query::*;

use cdrs::frame::IntoBytes;
use cdrs::types::from_cdrs::FromCDRSByName;
use cdrs::types::prelude::*;

type CurrentSession = Session<RoundRobin<TcpConnectionPool<StaticPasswordAuthenticator>>>;

#[derive(Debug, Clone, PartialEq, IntoCDRSValue, TryFromUDT)]
struct UserLogin {
    user_id: String,
}

#[derive(Debug, Clone, PartialEq, IntoCDRSValue, TryFromUDT)]
struct UserStore {
    pub user_id: String,
    pub name: String,
    pub roles: Vec<String>,
}

#[derive(Clone, Debug, IntoCDRSValue, TryFromRow, PartialEq)]
struct BlueStore {
    key: i32,
    users: HashMap<String, UserStore>,
    owners: Vec<UserLogin>,
    members: Vec<UserLogin>,
}

impl BlueStore {
    fn into_query_values(self) -> QueryValues {
        query_values!("key" => self.key,
            "users" => self.users,
            "owners" => self.owners,
            "members" => self.members)
    }
}

struct StoreConnection{
    session: CurrentSession,
}

impl StoreConnection{
    pub fn new() -> Self {
        let user = "user";
        let password = "password";
        let auth = StaticPasswordAuthenticator::new(&user, &password);
        let node = NodeTcpConfigBuilder::new("127.0.0.1:9042", auth).build();
        let cluster_config = ClusterTcpConfig(vec![node]);
        let session: CurrentSession =
            new_session(&cluster_config, RoundRobin::new()).expect("session should be created");

        StoreConnection { session }
    }
}

struct StoreManager{
    bp_name: String,
    connection: StoreConnection,
}

impl StoreManager {
    pub fn new(bp_name: &str, connection: StoreConnection) -> Self {
        StoreManager { bp_name: bp_name.to_string(),
            connection: connection}
    }

    fn session(&self) -> &CurrentSession {
        &self.connection.session
    }

    fn create_keyspace(&self) {
        let create_ks: &'static str = "CREATE KEYSPACE IF NOT EXISTS blues WITH REPLICATION = { \
                                   'class' : 'SimpleStrategy', 'replication_factor' : 1 };";
        self.session().query(create_ks).expect("Keyspace creation error");
    }

    fn create_udt(&self) {
        let create_type_cql = "CREATE TYPE IF NOT EXISTS blues.user_login (user_id text)";
        self.session()
            .query(create_type_cql)
            .expect("Keyspace creation error");
        let create_type_cql = "CREATE TYPE IF NOT EXISTS blues.user_store (\
            user_id text, name text, roles set<text>)";
        self.session()
            .query(create_type_cql)
            .expect("Keyspace creation error");
    }

    fn create_table(&self) {
        let create_table_cql =
            format!("CREATE TABLE IF NOT EXISTS blues.{} (key int PRIMARY KEY, \
         creator frozen<blues.user_store>, users map<text, frozen<blues.user_store>>, \
         owners list<frozen<blues.user_login>>, members set<frozen<blues.user_login>>);", self.bp_name);
        self.session()
            .query(create_table_cql)
            .expect("Table creation error");
    }

    pub fn create(&self){
        self.create_keyspace();
        self.create_udt();
        self.create_table();
    }
}

#[cfg(test)]
mod lib_tests {
    use super::*;

    #[test]
    fn bp_works() -> anyhow::Result<()> {
        let c=StoreConnection::new();
        let store=StoreManager::new("simple", c);
        store.create();

        Ok(())
    }

    #[test]
    fn append_works() -> anyhow::Result<()> {
        let c=StoreConnection::new();
        let store=StoreManager::new("simple", c);
        store.create();

        // ...
        let row = BlueStore {
            key: 3i32,
            users: hashmap! { "John".to_string() => UserStore {
                user_id: "John".to_string(),
                name: "John Smith".to_string(),
                roles: vec!["admin".to_string()]
            }},
            owners: vec![UserLogin {
                user_id: "John".to_string(),
            }],
            members: vec![UserLogin {
                user_id: "John".to_string(),
            }],
        };

        let insert_struct_cql = format!("INSERT INTO blues.{} \
                                 (key, users, owners, members) VALUES (?, ?, ?, ?)", store.bp_name);
        store.session()
            .query_with_values(insert_struct_cql, row.into_query_values())
            .expect("insert");
        // ...
        let key = 3i32;
        let extra_values = vec![
            UserLogin {
                user_id: "William".to_string(),
            },
            UserLogin {
                user_id: "Averel".to_string(),
            },
        ];
        let append_list_cql = format!("UPDATE blues.{} SET owners = owners + ? \
                           WHERE key = ?", store.bp_name);
        store.session()
            .query_with_values(append_list_cql, query_values!(extra_values, key))
            .expect("append owners");

        Ok(())
    }
}

