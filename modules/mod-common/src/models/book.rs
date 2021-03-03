use serde_derive::{Deserialize, Serialize};
use crate::schema::books;

#[derive(Serialize, Debug, Clone, Queryable)]
pub struct BookDTO {
    pub id: i64,
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

// Struct for creating Book
#[derive(Debug, Clone, Insertable)]
#[table_name = "books"]
pub struct CreateBookDTO {
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

// Handling enum as a text field in the database
use diesel::serialize::{ToSql, Output, IsNull};
use diesel::pg::Pg;
use std::io::Write;
use diesel::{serialize, deserialize};
use diesel::deserialize::FromSql;
use diesel::sql_types::Text;

#[derive(Serialize, Deserialize, Debug, Copy, Clone, AsExpression, FromSqlRow)]
#[sql_type = "Text"]
pub enum BookStatus {
    WantToRead,
    Reading,
    Finished,
    Rereading,
}

impl ToSql<Text, Pg> for BookStatus {
    fn to_sql<W: Write>(&self, out: &mut Output<W, Pg>) -> serialize::Result {
        match *self {
            BookStatus::WantToRead => out.write_all(b"WANT_TO_READ")?,
            BookStatus::Reading => out.write_all(b"READING")?,
            BookStatus::Finished => out.write_all(b"FINISHED")?,
            BookStatus::Rereading => out.write_all(b"REREADING")?,
        }
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Pg> for BookStatus {
    fn from_sql(bytes: Option<&[u8]>) -> deserialize::Result<Self> {
        match not_none!(bytes) {
            b"WANT_TO_READ" => Ok(BookStatus::WantToRead),
            b"READING" => Ok(BookStatus::Reading),
            b"FINISHED" => Ok(BookStatus::Finished),
            b"REREADING" => Ok(BookStatus::Rereading),
            _ => Err("Unrecognized enum variant".into()),
        }
    }
}

use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, PooledConnection};
use crate::errors::{AppError,ErrorType};

type PooledPg = PooledConnection<ConnectionManager<PgConnection>>;

pub struct BookManager {
    connection: PooledPg,
}

impl BookManager {
    pub fn new(connection: PooledPg) -> BookManager {
        BookManager {connection}
    }

    pub fn create_book(&self, dto: CreateBookDTO) -> Result<BookDTO, AppError> {
        use crate::schema::books;

        diesel::insert_into(books::table) // insert into books table
            .values(&dto) // use values from CreateBookDTO
            .get_result(&self.connection) // execute query
            .map_err(|err| {
                AppError::from_diesel_err(err, "while creating book")
            }) // if error occurred map it to AppError
    }

    pub fn list_books(&self) -> Result<Vec<BookDTO>, AppError> {
        use crate::schema::books::dsl::*;

        books
            .load(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while listing books")
            })
    }

    pub fn update_book_status(&self, book_id: i64, new_status: BookStatus) -> Result<usize, AppError> {
        use crate::schema::books::dsl::*;

        let updated = diesel::update(books)
            .filter(id.eq(book_id))
            .set(status.eq(new_status))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while updating book status")
            })?;

        if updated == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound))
        }
        return Ok(updated)
    }

    pub fn delete_book(&self, book_id: i64) -> Result<usize, AppError> {
        use crate::schema::books::dsl::*;

        let deleted = diesel::delete(books.filter(id.eq(book_id)))
            .execute(&self.connection)
            .map_err(|err| {
                AppError::from_diesel_err(err, "while deleting book")
            })?;

        if deleted == 0 {
            return Err(AppError::new("Book not found", ErrorType::NotFound))
        }
        return Ok(deleted)
    }
}

// api

#[derive(Debug, Deserialize, Clone)]
pub struct AddBook {
    pub title: String,
    pub author: String,
    pub status: BookStatus,
}

impl AddBook {
    pub fn to_dto(&self) -> CreateBookDTO {
        CreateBookDTO{
            title: self.title.clone(),
            author: self.author.clone(),
            status: self.status.clone(),
        }
    }
}

#[derive(Debug, Deserialize, Clone)]
pub struct UpdateStatus {
    pub status: BookStatus,
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{establish_connection, establish_connection_with_pool};
    use crate::api::IdResponse;

    #[test]
    fn book_works() {
        let connection = establish_connection_with_pool();
        let db_manager=BookManager{ connection };

        let new_book: AddBook=AddBook{
            title: "hello".to_string(),
            author: "samlet".to_string(),
            status: BookStatus::WantToRead
        };
        let create_book_dto = new_book.to_dto();
        let id_response = db_manager.create_book(create_book_dto).map(|book|
            { IdResponse::new(book.id) }
        );

        println!("{:?}", id_response);
    }
}
