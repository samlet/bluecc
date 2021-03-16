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
