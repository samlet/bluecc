use std::sync::Arc;
use std::collections::HashMap;

#[derive(Clone)]
pub struct User {
    pub uid: String,
    pub email: String,
    pub pw: String,
    pub role: String,
}

pub type Users = Arc<HashMap<String, User>>;

pub fn init_users() -> HashMap<String, User> {
    let mut map = HashMap::new();
    map.insert(
        String::from("1"),
        User {
            uid: String::from("1"),
            email: String::from("user@userland.com"),
            pw: String::from("1234"),
            role: String::from("User"),
        },
    );
    map.insert(
        String::from("2"),
        User {
            uid: String::from("2"),
            email: String::from("admin@adminaty.com"),
            pw: String::from("4321"),
            role: String::from("Admin"),
        },
    );
    map
}

