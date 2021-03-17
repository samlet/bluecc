use crate::conn::establish_connection;
use diesel::prelude::*;
use serde_json::json;
use seed::{new_snowflake_id, GenericError};
use bcrypt::*;
use diesel::{self, insert_into, delete, sql_query};

use seed::schema::users;
use inflector::Inflector;
use diesel::deserialize::FromSql;
use diesel::backend::Backend;
use chrono::NaiveDateTime;

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

#[derive(Queryable)]
pub struct UserRec {
    pub id: i32,
    pub username: String,
}

fn get_user(conn: &PgConnection, id: i32) -> Result<UserRec, GenericError> {
    let user = users::table
        .filter(users::id.eq(id))
        .select((users::id, users::username))
        .first::<UserRec>(conn)?;
    Ok(user)
}

#[test]
fn find_by_id_works() -> Result<(), GenericError>  {
    let conn = establish_connection();
    let user=get_user(&conn, 1)?;
    println!("user name {}", user.username);

    if let Ok(user)=get_user(&conn, 8888) {
        println!("user name {}", user.username);
    }
    Ok(())
}

mod relates {
    use crate::conn::establish_connection;
    use diesel::prelude::*;
    use chrono::NaiveDateTime;
    use seed::schema::{users, posts};
    use seed::{new_snowflake_id, GenericError};

    use diesel::pg::Pg;
    use diesel::query_source::Queryable;
    use diesel::sql_types::{Nullable, Timestamp};

    #[derive(Debug, Serialize)]
    pub enum Status {
        Draft,
        Published { at: NaiveDateTime },
    }

    impl Queryable<Nullable<Timestamp>, Pg> for Status {
        type Row = Option<NaiveDateTime>;

        fn build(row: Self::Row) -> Self {
            match row {
                Some(at) => Status::Published { at },
                None => Status::Draft,
            }
        }
    }

    #[derive(Queryable, Associations, Identifiable, Debug, Serialize)]
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

    #[derive(Queryable, Identifiable, Debug, PartialEq)]
    pub struct User {
        pub id: i32,
        pub username: String,
    }

    fn get_user_by_id(conn: &PgConnection, id: i32) -> Result<User, GenericError> {
        let user = users::table
            .filter(users::id.eq(id))
            .select((users::id, users::username))
            .first::<User>(conn)?;
        Ok(user)
    }

    #[test]
    fn relates_works() -> anyhow::Result<()> {
        let conn = establish_connection();

        let user = get_user_by_id(&conn, 2)?;
        let body = "simple content";
        let post_id = diesel::insert_into(posts::table)
            .values((
                posts::user_id.eq(user.id),
                posts::title.eq("just a test blog"),
                posts::body.eq(body),
            ))
            .returning(posts::id)
            .get_result::<i32>(&conn)?;
        println!("create a post {}", post_id);

        let post = Post::belonging_to(&user)
            .find(post_id)
            .first::<Post>(&conn)?;
        let post_json=serde_json::to_string_pretty(&post)?;
        println!("post: {}", post_json);

        println!("** find all posts belongs to the user {}", user.id);
        let posts = Post::belonging_to(&user)
            .load::<Post>(&conn)?;
        for post in &posts {
            let post_json = serde_json::to_string_pretty(post)?;
            println!("post: {}", post_json);
        }

        Ok(())
    }
}

