use serde_derive::{Deserialize, Serialize};
use crate::schema::mnemonics;
use diesel::prelude::*;

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct Mnemonic {
    pub id: i32,
    pub path: String,
    pub num_value: i32,
    pub string_value: String,
    pub description: Option<String>,
}

#[derive(Debug, Insertable)]
#[table_name = "mnemonics"]
pub struct NewMnemonic {
    pub path: String,
    pub num_value: i32,
    pub string_value: String,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection;
    use crate::schema::mnemonics;

    #[test]
    fn mnemonic_works() {
        let conn = establish_connection();

        let new_post = NewMnemonic{
            path: "example:example_id".to_string(),
            num_value: 1,
            string_value: "EX01".to_string()
        };

        let new_rec=diesel::insert_into(mnemonics::table)
            .values(&new_post)
            .get_result::<Mnemonic>(&conn)
            .expect("Error saving new record");

        println!("{:?}", new_rec);
    }

    #[test]
    fn query_works() {
        use crate::schema::mnemonics::dsl::*;
        let conn = establish_connection();
        let results=mnemonics.filter(path.eq("example:example_id"))
            .limit(5).load::<Mnemonic>(&conn)
            .expect("Error loading");
        for r in results{
            println!("{:?}", r);
        }
    }

    #[test]
    fn query_first_works() {
        use crate::schema::mnemonics::dsl::*;
        let conn = establish_connection();
        let result:Mnemonic=mnemonics
            .filter(path.eq("example:example_id"))
            .filter(string_value.eq("EX01"))
            .first(&conn)
            .expect("Error loading");
        println!("{} -> {:?}", result.id, result);
    }

    #[test]
    fn exists_works() {
        use crate::schema::mnemonics::dsl::*;
        use diesel::dsl::exists;

        let connection = establish_connection();
        let rec_exists = diesel::select(exists(mnemonics
            .filter(path.eq("example:example_id"))
            .filter(string_value.eq("EX01"))
        ))
            .get_result(&connection);
        assert_eq!(Ok(true), rec_exists);

    }

    #[test]
    fn select_works() -> anyhow::Result<()>{
        use crate::schema::mnemonics::dsl::*;
        let connection = establish_connection();

        let results:Vec<_> = mnemonics.select((path, string_value, num_value))
            .filter(path.eq("example:example_id"))
            .filter(string_value.eq("EX01"))
            .load::<(String, String, i32)>(&connection)?;

        assert!(results.len()>0);
        for r in results{
            println!("{}/{} => {:?}", r.0, r.1, r);
        }
        Ok(())
    }
}



