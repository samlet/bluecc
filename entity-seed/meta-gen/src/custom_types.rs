use crate::conn::establish_connection;
use diesel::prelude::*;
use serde_json::json;
use seed::new_snowflake_id;
use bcrypt::*;
use diesel::{self, insert_into, delete, sql_query};

use seed::schema::users;
use inflector::Inflector;
use diesel::deserialize::FromSql;
use diesel::backend::Backend;

#[test]
fn user_works() -> anyhow::Result<()> {

    #[derive(Queryable, Identifiable, Debug, PartialEq)]
    pub struct User {
        pub id: i32,
        pub username: String,
    }

    let conn = establish_connection();

    let username=format!("user_{:?}", new_snowflake_id());
    let password="1234";
    let hashed_password = hash(password, DEFAULT_COST)?;
    let result:(i32, String)=insert_into(users::table)
        .values((
            users::username.eq(username),
            users::hashed_password.eq(hashed_password),
        ))
        .returning((users::id, users::username))
        .get_result(&conn)?;
    println!("{:?}", result);

    let rec:User=users::table.filter(users::id.eq(result.0))
        .select((users::id, users::username))
        .first(&conn)?;
    println!("{:?}", rec);

    // change user-name
    let new_name=format!("new_user_{:?}", new_snowflake_id());
    diesel::update(users::table)
        .filter(users::id.eq(result.0))
        .set(users::username.eq(new_name))
        .execute(&conn)?;

    let rec:User=users::table
        .filter(users::id.eq(result.0))
        .select((users::id, users::username))
        .first(&conn)?;
    println!("{:?}", rec);

    println!("remove {:?}", result.0);
    delete(users::table)
        .filter(users::id.eq(result.0))
        .execute(&conn)?;

    Ok(())
}

#[derive(PartialEq, Debug)]
struct User {
    id: i32,
    name: String,
}

/// ref: http://docs.diesel.rs/diesel/deserialize/trait.Queryable.html
type DB = diesel::pg::Pg;
impl Queryable<users::SqlType, DB> for User {
    type Row = (i32, String, String, chrono::NaiveDateTime, chrono::NaiveDateTime);

    fn build(row: Self::Row) -> Self {
        User {
            id: row.0,
            name: row.1.to_title_case(),
        }
    }
}

#[test]
fn find_with_cust_types_works() -> anyhow::Result<()> {
    let conn = establish_connection();

    let first_user:User = users::table.first(&conn)?;
    // let expected = User { id: 1, name: "sean".into() };
    // assert_eq!(expected, first_user);
    println!("{:?}, title-case: {}", first_user, first_user.name.is_title_case());

    Ok(())
}

/// ref: http://docs.diesel.rs/diesel/deserialize/trait.QueryableByName.html
struct LowercaseString(String);

impl Into<String> for LowercaseString {
    fn into(self) -> String {
        self.0
    }
}

impl<DB, ST> FromSql<ST, DB> for LowercaseString
where
    DB: Backend,
    String: FromSql<ST, DB>,
{
    fn from_sql(bytes: Option<&DB::RawValue>) -> diesel::deserialize::Result<Self> {
        String::from_sql(bytes)
            .map(|s| LowercaseString(s.to_lowercase()))
    }
}

#[derive(QueryableByName, PartialEq, Debug)]
#[table_name = "users"]
struct UserName {
    id: i32,
    #[diesel(deserialize_as = "LowercaseString")]
    username: String,
}

#[test]
fn query_by_name_works() -> anyhow::Result<()> {
    let conn = establish_connection();
    let first_user:UserName = sql_query("SELECT * FROM users ORDER BY id LIMIT 1")
        .get_result(&conn)?;
    let expected = UserName { id: 1, username: "sgrif".into() };
    assert_eq!(expected, first_user);
    Ok(())
}

/// ref: http://docs.diesel.rs/diesel/dsl/fn.insert_into.html
#[test]
fn insert_multi_works() -> anyhow::Result<()> {
    use seed::schema::users::dsl::*;

    let conn = establish_connection();

    let password="1234";
    let pass_hash = hash(password, DEFAULT_COST)?;

    let new_users = vec![
        (id.eq(2222), username.eq("Tess"), hashed_password.eq(&pass_hash)),
        (id.eq(2223), username.eq("Jim"), hashed_password.eq(&pass_hash)),
    ];

    let rows_inserted = diesel::insert_into(users)
        .values(&new_users)
        .execute(&conn);

    assert_eq!(Ok(2), rows_inserted);
    Ok(())
}

#[derive(PartialEq, Insertable, Deserialize, Serialize, AsChangeset, Clone)]
#[table_name = "users"]
pub struct UserF {
    id: i32,
    username: String,
    hashed_password: String,
}

#[test]
fn insert_or_ignore_into_works() -> anyhow::Result<()> {
    use diesel::dsl::{insert_or_ignore_into, replace_into};
    use seed::schema::users::dsl::*;

    let password="1234";
    let pass_hash = hash(password, DEFAULT_COST)?;

    let conn = establish_connection();
    // With PostgreSQL, similar functionality is provided by on_conflict_do_nothing.
    // insert_or_ignore_into(users)
    insert_into(users)
        .values((id.eq(1), username.eq("Jim"), hashed_password.eq(&pass_hash)))
        .on_conflict(id)
        .do_nothing()
        .execute(&conn)?;

    // insert_or_ignore_into(users)
    insert_into(users)
    // replace_into, insert_or_ignore_into: This function is only available with MySQL and SQLite.
    // replace_into(users)
        .values(&vec![
            (id.eq(1), username.eq("Sean"), hashed_password.eq(&pass_hash)),
            (id.eq(2), username.eq("Tess"), hashed_password.eq(&pass_hash)),
        ])
        .on_conflict(id)
        .do_nothing()
        .execute(&conn)?;

    // let user = UserF { id: 1, username: "Pascal".to_string(), hashed_password: pass_hash.to_owned() };
    let user2 = UserF { id: 1,
        username: format!("Sean_{}", new_snowflake_id()),
        hashed_password: pass_hash.to_owned()
    };

    // assert_eq!(Ok(1), diesel::insert_into(users).values(&user).execute(&conn));

    let _insert_count = diesel::insert_into(users)
        .values(&user2)
        .on_conflict(id)
        .do_update()
        .set(&user2)
        .execute(&conn);

    let names = users.select((id, username)).order(id).load::<(i32,String)>(&conn)?;
    // assert_eq!(vec![String::from("Jim"), String::from("Tess")], names);
    for n in names{
        println!("- {}. {}", n.0, n.1);
    }
    Ok(())
}
