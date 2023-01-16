use crate::lib::MONGODB_CLIENT;
use futures::TryStreamExt;
use mongodb::{
    bson::{doc, Bson},
    error::Error,
    options::FindOptions,
    Collection,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ArticleDoc {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub _id: Option<Bson>,
    pub title: String,
    pub description: Option<String>,
    pub summary: Option<String>,
    pub content: Option<String>,
    pub is_delete: bool,
    pub create_time: i64,
    pub update_time: i64,
}

impl ArticleDoc {
    pub fn new() -> Self {
        Self {
            _id: None,
            title: "".to_string(),
            description: None,
            summary: None,
            content: None,
            is_delete: false,
            create_time: 0,
            update_time: 0,
        }
    }

    pub async fn create(doc: ArticleDoc) -> Result<String, Error> {
        let collection = Self::get_collection();

        let r = collection.insert_one(doc, None).await;
        match r {
            Err(e) => Err(e),
            Ok(insert_one_result) => Ok(insert_one_result
                .inserted_id
                .as_object_id()
                .unwrap()
                .to_string()),
        }
    }

    pub async fn update_by_id(
        id: &String,
        title: &String,
        desc: &Option<String>,
        summary: &Option<String>,
        content: &Option<String>,
        is_delete: bool,
    ) -> Result<(), Error> {
        let collection = Self::get_collection();

        let query = doc! {"_id": id};
        let update = doc! {
            "update_time": chrono::Local::now().timestamp_millis(),
            "is_delete": is_delete,
            "title": title,
            "description": desc,
            "summary": summary,
            "content": content,
        };

        let update_r = collection.update_one(query, update, None).await;

        match update_r {
            Err(e) => Err(e),
            Ok(_) => Ok(()),
        }
    }

    pub async fn find_by_ids(ids: &Vec<Bson>) -> Result<Vec<ArticleDoc>, Error> {
        let collection = Self::get_collection();

        let filter = doc! { "_id": {"$in": ids}};

        let find_ops = FindOptions::builder()
            .sort(doc! {"update_time": -1})
            .build();

        let find_r = collection.find(filter, find_ops).await;

        match find_r {
            Err(e) => Err(e),
            Ok(mut cursor) => {
                let mut vec: Vec<ArticleDoc> = vec![];
                while let Some(article) = cursor.try_next().await? {
                    vec.push(article.clone());
                }

                Ok(vec)
            }
        }
    }

    fn get_collection() -> Collection<ArticleDoc> {
        MONGODB_CLIENT
            .clone()
            .default_database()
            .unwrap()
            .collection::<ArticleDoc>("article")
    }
}
