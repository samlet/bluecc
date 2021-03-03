use crate::schema::posts;
use diesel::prelude::*;

#[derive(Queryable)]
pub struct Post {
    pub id: i32,
    pub title: String,
    pub body: String,
    pub published: bool,
}

#[derive(Insertable)]
#[table_name = "posts"]
pub struct NewPost<'a> {
    pub title: &'a str,
    pub body: &'a str,
}


pub fn create_post(conn: &PgConnection, title: &str, body: &str) -> Post {
    use crate::schema::posts;

    let new_post = NewPost { title, body };

    diesel::insert_into(posts::table)
        .values(&new_post)
        .get_result(conn)
        .expect("Error saving new post")
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::establish_connection;

    #[test]
    fn post_works() {
        let connection = establish_connection();
        let title="demo";
        let body="demo content";
        let post = create_post(&connection, title, &body);
        println!("\nSaved draft {} with id {}", title, post.id);
    }

    #[test]
    fn all_posts() {
        use crate::schema::posts::dsl::*;

        let connection = establish_connection();
        let results = posts
            // .filter(published.eq(true))
            .limit(5)
            .load::<Post>(&connection)
            .expect("Error loading posts");

        println!("Displaying {} posts", results.len());
        for post in results {
            println!("{}", post.title);
            println!("-----------\n");
            println!("{}", post.body);
        }
    }
}

