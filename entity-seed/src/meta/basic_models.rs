use diesel::prelude::*;
use diesel::{self, insert_into};
use dotenv;
use chrono::NaiveDateTime;
use bcrypt::*;

use crate::schema::posts;
use crate::schema::users;


#[derive(Debug)]
pub enum AuthenticationError {
    IncorrectPassword,
    NoUsernameSet,
    NoPasswordSet,
    EnvironmentError(dotenv::Error),
    BcryptError(BcryptError),
    DatabaseError(diesel::result::Error),
}

impl From<BcryptError> for AuthenticationError {
    fn from(e: BcryptError) -> Self {
        AuthenticationError::BcryptError(e)
    }
}

pub use self::AuthenticationError::{IncorrectPassword, NoPasswordSet, NoUsernameSet};


#[derive(Queryable, Identifiable, Debug, PartialEq)]
pub struct User {
    pub id: i32,
    pub username: String,
}

#[derive(Queryable)]
pub struct UserWithPassword {
    user: User,
    password: String,
}

#[derive(Queryable, Associations, Identifiable)]
#[belongs_to(User)]
pub struct Post {
    pub id: i32,
    pub user_id: i32,
    pub title: String,
    pub body: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
    pub status: Status,
}

pub enum Status {
    Draft,
    Published { at: NaiveDateTime },
}


pub fn current_user_from_env(conn: &PgConnection) -> Result<Option<User>, AuthenticationError> {
    let username = get_username()?;
    let password = get_password()?;
    find_user(conn, &username, &password)
}

pub fn register_user_from_env(conn: &PgConnection) -> Result<User, AuthenticationError> {
    let username = get_username()?;
    let password = get_password()?;
    register_user(conn, &username, &password)
}

fn find_user(
    conn: &PgConnection,
    username: &str,
    password: &str,
) -> Result<Option<User>, AuthenticationError> {
    let user_and_password = users::table
        .filter(users::username.eq(username))
        .select(((users::id, users::username), users::hashed_password))
        .first::<UserWithPassword>(conn)
        .optional()
        .map_err(AuthenticationError::DatabaseError)?;

    if let Some(user_and_password) = user_and_password {
        if verify(password, &user_and_password.password)? {
            Ok(Some(user_and_password.user))
        } else {
            Err(IncorrectPassword)
        }
    } else {
        Ok(None)
    }
}

fn register_user(
    conn: &PgConnection,
    username: &str,
    password: &str,
) -> Result<User, AuthenticationError> {
    let hashed_password = hash(password, DEFAULT_COST)?;
    insert_into(users::table)
        .values((
            users::username.eq(username),
            users::hashed_password.eq(hashed_password),
        ))
        .returning((users::id, users::username))
        .get_result(conn)
        .map_err(AuthenticationError::DatabaseError)
}

fn get_username() -> Result<String, AuthenticationError> {
    if_not_present(dotenv::var("SEED_USERNAME"), NoUsernameSet)
}

fn get_password() -> Result<String, AuthenticationError> {
    if_not_present(dotenv::var("SEED_PASSWORD"), NoPasswordSet)
}

fn if_not_present<T>(
    res: Result<T, dotenv::Error>,
    on_not_present: AuthenticationError,
) -> Result<T, AuthenticationError> {
    use dotenv::ErrorKind::EnvVar;
    use std::env::VarError::NotPresent;

    res.map_err(|e| match e {
        dotenv::Error(EnvVar(NotPresent), _) => on_not_present,
        e => AuthenticationError::EnvironmentError(e),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use crate::meta::test_helpers::{this_test_modifies_env, test_connection, establish_connection_with_pool};

    #[test]
    fn current_user_from_env_fails_when_no_username_set() {
        let _guard = this_test_modifies_env();
        env::remove_var("SEED_USERNAME");

        let conn = test_connection();

        assert_matches!(current_user_from_env(&conn), Err(NoUsernameSet));
    }


    #[test]
    fn current_user_from_env_fails_when_no_password_set() {
        let _guard = this_test_modifies_env();
        env::remove_var("SEED_PASSWORD");
        env::set_var("SEED_USERNAME", "sgrif");

        let conn = test_connection();

        assert_matches!(current_user_from_env(&conn), Err(NoPasswordSet));
    }

    #[test]
    fn current_user_returns_none_when_no_user_exists_with_username() {
        let conn = test_connection();

        assert_matches!(find_user(&conn, "sgrif", "hunter2"), Ok(None));
    }

    #[test]
    fn current_user_returns_the_user_if_it_has_the_same_password() {
        let conn = test_connection();

        let expected_user = register_user(&conn, "sgrif", "hunter2").unwrap();
        let user = find_user(&conn, "sgrif", "hunter2").unwrap();

        assert_eq!(Some(expected_user), user);
    }

    #[test]
    fn real_check_user() {
        let conn = establish_connection_with_pool();
        let user = find_user(&conn, "sgrif", "hunter2").unwrap();
        match user {
            Some(u) => println!("user exists: {:?}", u),
            None => {
                let expected_user = register_user(&conn, "sgrif", "hunter2").unwrap();
                println!("{:?}", expected_user);
                assert_eq!(expected_user.username, user.unwrap().username);
            }
        }
    }

    #[test]
    fn current_user_fails_if_password_does_not_match() {
        let conn = test_connection();

        register_user(&conn, "sgrif", "letmein").unwrap();
        let result = find_user(&conn, "sgrif", "hunter2");

        assert_matches!(result, Err(IncorrectPassword));
    }
}
