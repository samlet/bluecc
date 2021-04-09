use mongodb::bson::{doc, document::Document, oid::ObjectId, Bson};
use mongodb::{options::ClientOptions, Client, Collection};
use common::prelude::*;
use chrono::prelude::*;

const ID: &str = "_id";

#[derive(Serialize, Deserialize, Debug)]
struct DispatStatus {
    status: String,
    origin: String,
}

#[cfg(test)]
mod lib_tests {
    use super::*;
    use futures::StreamExt;
    use mongodb::bson;

    macro_rules! assert_coll_count {
        ($coll:expr, $expected:expr) => {
            assert_eq!($coll.count_documents(None, None).await.unwrap(), $expected);
        };
    }

    macro_rules! run_on_each_doc {
        ($cursor:expr, $name:ident, $check:block) => {{
            let mut cursor = $cursor;

            while let Some($name) = cursor.try_next().await.unwrap() $check;
        }};
    }

    #[tokio::test]
    async fn connect_works() -> anyhow::Result<()> {
        let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
        let db = client.database("some_db");
        for coll_name in db.list_collection_names(None).await? {
            println!("collection: {}", coll_name);
        }

        let coll = db.collection("some-coll");
        let result = coll.insert_one(doc! { "x": 1 }, None).await?;
        println!("{:#?}", result);
        Ok(())
    }

    #[tokio::test]
    async fn quote_works() -> anyhow::Result<()> {
        let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
        let db = client.database("some_db");
        let collection = db.collection("quote");
        collection.drop(None).await?;

        collection.insert_one(doc! {
              "quoteTypeId": "PRODUCT_QUOTE",
              "partyId": "DemoCustomer",
              "salesChannelEnumId": "PHONE_SALES_CHANNEL",
              "statusId": "QUO_CREATED",
              "validThruDate": from_std_fmt("2100-02-01 00:00:00.0")?,
              "productStoreId": "9000",
              "validFromDate": Utc::now(),
              "issueDate": from_std_fmt("2001-01-01 00:00:00.0")?,
              "quoteName": "Demo Quote",
              "currencyUomId": "USD",
              "quoteId": "CQ0001"
            }, None).await?;

        // assert_coll_count!(collection, 1);

        let mut cursor = collection.find(
            doc! { "quoteId": "CQ0001" },
            None).await?.enumerate();
        while let Some((i, result)) = cursor.next().await {
            let doc = result?;
            println!("doc.{} len {}", i, doc.len());
            let id = doc.get_object_id(ID)?;
            let party_id = doc.get_str("partyId")?;
            let issue_date = doc.get_datetime("issueDate")?;
            println!("{} - {}, {}", id.to_hex(), party_id, issue_date);
        }

        Ok(())
    }

    #[tokio::test]
    async fn quote_term_works() -> anyhow::Result<()> {
        let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
        let db = client.database("some_db");
        let collection = db.collection("quote_term");
        collection.drop(None).await?;

        collection.insert_one(doc! {
              "quoteId": "CQ0001",
              "quoteItemSeqId": "_NA_",
              "termDays": 30 as i32,
              "termTypeId": "FIN_PAYMENT_TERM"
            }, None).await?;

        // assert_coll_count!(collection, 1);

        let mut cursor = collection.find(
            doc! { "quoteId": "CQ0001" },
            None).await?.enumerate();
        while let Some((i, result)) = cursor.next().await {
            let doc = result?;
            println!("doc.{} len {}", i, doc.len());
            let id = doc.get_object_id(ID)?;
            let quote_id = doc.get_str("quoteId")?;
            let term_days = doc.get_i32("termDays")?;
            println!("{} - {}, {}", id.to_hex(), quote_id, term_days);
        }

        Ok(())
    }

    #[tokio::test]
    async fn quote_item_works() -> anyhow::Result<()> {
        let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
        let db = client.database("some_db");
        let collection = db.collection("quote_item");
        collection.drop(None).await?;

        let insert_result=collection.insert_one(doc! {
            "productId": "GZ-1000",
            "quantity": 150.0,
            "comments": "10% off",
            "quoteItemSeqId": "00001",
            "quoteUnitPrice": 9.9,
            "quoteId": "CQ0001",
            "_dispat": {
                "status": "pending",
                "origin": "b_0001"
            }
        }, None).await?;

        // assert_coll_count!(collection, 1);

        let mut cursor = collection.find(
            doc! { "quoteId": "CQ0001" },
            None).await?.enumerate();
        while let Some((i, result)) = cursor.next().await {
            let doc = result?;
            println!("doc.{} len {}", i, doc.len());
            let id = doc.get_object_id(ID)?;
            let quote_id = doc.get_str("quoteId")?;
            let quote_unit_price = doc.get_f64("quoteUnitPrice")?;
            println!("{} - {}, {}", id.to_hex(), quote_id, quote_unit_price);

            if let Some(v) = doc.get("comments").and_then(Bson::as_str) {
                println!("comments: {}", v);
            } else {
                println!("no comments found");
            }

            if let Some(v) = doc.get("_dispat").and_then(Bson::as_document) {
                println!("_dispat: {}", v);
                let loaded_dispat: DispatStatus = bson::from_document(v.to_owned())?;
                println!("_dispat_m: {:?}", loaded_dispat);
            } else {
                println!("no _dispat found");
            }
        }

        // ...
        // Look up one document:
        let r = collection.find_one(doc! {
            "quoteId": "CQ0001",
            "quoteItemSeqId": "00001",
        }, None).await?
           .expect("Missing 'CQ0001' document.");
        println!("QuoteItem: {}", r);

        // Update the document dispat-status:
        let update_result = collection.update_one(
           doc! {
              "_id": &insert_result.inserted_id,
           },
           doc! {
              "$set": { "_dispat.status": "success" }
           },
           None,
        ).await?;
        println!("Updated {} document", update_result.modified_count);

        // Delete all documents for status 'success'
        let delete_result = collection.delete_many(
           doc! {
              "_dispat.status": "success"
           },
           None,
        ).await?;
        println!("Deleted {} documents", delete_result.deleted_count);

        Ok(())
    }

    #[derive(Debug, Serialize, Deserialize)]
    struct Student {
        name: String,
        grade: i32,
        test_scores: Vec<i32>,
    }

    #[tokio::test]
    async fn serde_works() -> anyhow::Result<()> {
        let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
        let db = client.database("some_db");
        let students: Collection<Student> = db.collection_with_type("students");
        students.drop(None).await?;

        let student = Student {
            name: "Emily".to_string(),
            grade: 10,
            test_scores: vec![98, 87, 100],
        };
        let result = students.insert_one(student, None).await?;
        println!("{:?}", result);

        // student is of type Student
        let student = students.find_one(doc! { "name": "Emily" }, None).await?;
        println!("{}", pretty(&student.unwrap()));

        Ok(())
    }

    #[derive(Serialize, Deserialize, Debug)]
    struct Movie {
       #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
       id: Option<bson::oid::ObjectId>,
       title: String,
       year: i32,
    }

    /// https://developer.mongodb.com/quickstart/rust-crud-tutorial/
    /// https://developer.mongodb.com/article/serde-improvements/
    #[tokio::test]
    async fn serde_with_id_works() -> anyhow::Result<()> {
        let client = Client::with_uri_str("mongodb://localhost:27017/").await?;
        let db = client.database("some_db");
        let movies = db.collection("movies");
        movies.drop(None).await?;

        // Initialize struct to be inserted:
        let captain_marvel = Movie {
            id: None,
            title: "Captain Marvel".to_owned(),
            year: 2019,
        };

        // Convert `captain_marvel` to a Bson instance:
        let serialized_movie = bson::to_bson(&captain_marvel)?;
        let document = serialized_movie.as_document().unwrap();
        // Insert into the collection and extract the inserted_id value:
        let insert_result = movies.insert_one(document.to_owned(), None).await?;
        let captain_marvel_id = insert_result
            .inserted_id
            .as_object_id()
            .expect("Retrieved _id should have been of type ObjectId");
        println!("Captain Marvel document ID: {:?}", captain_marvel_id);

        // Retrieve Captain Marvel from the database, into a Movie struct:
        // Read the document from the movies collection:
        let loaded_movie = movies
           .find_one(Some(doc! { "_id":  captain_marvel_id.clone() }), None)
           .await?
           .expect("Document not found");

        // Deserialize the document into a Movie instance
        let loaded_movie_struct: Movie = bson::from_bson(Bson::Document(loaded_movie))?;
        println!("Movie loaded from collection: {:?}", loaded_movie_struct);

        Ok(())
    }

}


