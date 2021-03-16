use crate::conn::establish_connection;
use diesel::prelude::*;
use serde_json::json;
use seed::new_snowflake_id;

#[test]
fn user_login_gen_works() -> anyhow::Result<()> {
    use seed::models::security_types::UserLogin;
    use seed::schema::user_login::dsl::*;

    let conn = establish_connection();
    let json = json!(
        {
          "userLoginId": new_snowflake_id(),
          "currentPassword": null,
          "passwordHint": null,
          "isSystem": true,
          "enabled": false,
          "disabledBy": null
        }
    );
    let rec = serde_json::from_value::<UserLogin>(json)?;
    diesel::insert_into(user_login).values(&rec).execute(&conn)?;
    Ok(())
}

#[test]
fn list_user_logins_cols() -> anyhow::Result<()> {
    use seed::schema::user_login::dsl::*;

    let conn = establish_connection();
    let rs=user_login.select((user_login_id, current_password.nullable()))
        .order(user_login_id.desc())
        .load::<(i64, Option<String>)>(&conn)?;

    for r in &rs{
        println!("{:?}", r);
    }
    Ok(())
}

#[test]
fn list_recs() -> anyhow::Result<()> {
    use seed::schema::uom_type::dsl::*;
    use seed::schema::user_login::dsl::*;
    use seed::models::common::UomType;
    use seed::models::security::UserLogin;

    let conn = establish_connection();

    let total=uom_type
            .count()
            .get_result::<i64>(&conn);
    println!("total {:?}", total);

    println!("list uom_type recs");
    let rs:Vec<UomType>=uom_type
                .load(&conn)?;
    for r in &rs{
        println!("{:?}", r);
    }

    println!("list user_login recs");
    let rs:Vec<UserLogin>=user_login
                .load(&conn)?;
    for r in &rs{
        println!("{:?}", r);
    }
    Ok(())
}


