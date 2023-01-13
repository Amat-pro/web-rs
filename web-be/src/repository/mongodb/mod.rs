use crate::lib::MONGODB_CLIENT;
use mongodb::bson::{doc, Document};

async fn demo() {
    // Get a handle to a collection in the database.
    let collection = MONGODB_CLIENT
        .clone()
        .default_database()
        .unwrap()
        .collection::<Document>("books");

    let docs = vec![
        doc! { "title": "1984", "author": "George Orwell" },
        doc! { "title": "Animal Farm", "author": "George Orwell" },
        doc! { "title": "The Great Gatsby", "author": "F. Scott Fitzgerald" },
    ];

    // Insert some documents into the "mydb.books" collection.
    let r = collection.insert_many(docs, None).await;

    match r {
        Ok(ok) => {
            println!("{:?}", ok.inserted_ids);
        }
        Err(e) => println!("{}", e),
    }
}
