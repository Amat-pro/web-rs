use crate::repository::mongodb as repo_mongo;
use crate::repository::mysql;
use crate::structs::{
    global_response, new_page, Article, ArticleCreateAO, ArticleCreateVO, ArticleListMineAO,
    ArticleUpdateAO, ArticleUpdateVO, UserInfo,
};
use axum::extract::Json;
use mongodb::bson::{oid::ObjectId, Bson};
use serde_json::Value;
use std::str::FromStr;
use tracing::warn;

#[tracing::instrument]
pub async fn create(user: &UserInfo, ao: &ArticleCreateAO) -> Json<Value> {
    let ArticleCreateAO {
        title,
        description,
        summary,
        content,
        ..
    } = ao;

    let timestamp_millis = chrono::Local::now().timestamp_millis();

    let mut article_doc = repo_mongo::ArticleDoc::new();
    article_doc.title = title.clone();
    article_doc.description = description.clone();
    article_doc.summary = summary.clone();
    article_doc.content = content.clone();
    article_doc.is_delete = false;
    article_doc.create_time = timestamp_millis;
    article_doc.update_time = timestamp_millis;

    let create_r = repo_mongo::ArticleDoc::create(article_doc).await;
    if create_r.is_err() {
        warn!("mongodb create article err: {}", create_r.err().unwrap());
        return global_response::new(
            global_response::ERROR_CODE_SERVER_ERROR,
            ArticleCreateVO::new(),
        );
    }

    let article_id = create_r.unwrap();
    let user_id = user.id.parse::<i64>().unwrap();

    let create_r2 = mysql::UserArticleEntity::create(&article_id, user_id).await;
    if create_r2.is_err() {
        warn!("mysql create article err: {}", create_r2.err().unwrap());
        return global_response::new(
            global_response::ERROR_CODE_SERVER_ERROR,
            ArticleCreateVO::new(),
        );
    }

    global_response::new(global_response::ERROR_CODE_SUCCESS, ArticleCreateVO::new())
}

#[tracing::instrument]
pub async fn update(user: &UserInfo, ao: &ArticleUpdateAO) -> Json<Value> {
    let ArticleUpdateAO {
        id,
        title,
        description,
        summary,
        content,
    } = ao;

    let update_r =
        repo_mongo::ArticleDoc::update_by_id(id, title, description, summary, content, false).await;

    if update_r.is_err() {
        warn!(
            "mongodb update_by_id article err: {}",
            update_r.err().unwrap()
        );
        return global_response::new(
            global_response::ERROR_CODE_SERVER_ERROR,
            ArticleCreateVO::new(),
        );
    }

    global_response::new(global_response::ERROR_CODE_SUCCESS, ArticleUpdateVO::new())
}

#[tracing::instrument]
pub async fn list_mine(user: &UserInfo, ao: &ArticleListMineAO) -> Json<Value> {
    let page_param = ao.page_param.clone().unwrap();

    let user_id = user.id.parse::<i64>().unwrap();
    let find_r = mysql::UserArticleEntity::find_page_by_user_id(user_id, page_param.clone()).await;
    if find_r.is_err() {
        warn!(
            "list_mine find_page_by_user_id from mysql err: {}",
            find_r.err().unwrap()
        );
        return global_response::new(
            global_response::ERROR_CODE_SERVER_ERROR,
            ArticleCreateVO::new(),
        );
    }

    let article_page = find_r.unwrap();
    if article_page.total <= 0 {
        return global_response::new(
            global_response::ERROR_CODE_SUCCESS,
            new_page::<Vec<Article>>(page_param.size, page_param.current, 0, vec![]),
        );
    }

    let mut article_ids: Vec<Bson> = vec![];
    for article in article_page.data {
        let id = Bson::ObjectId(ObjectId::from_str(article.article_id.as_str()).unwrap());
        article_ids.push(id);
    }

    let find_r2 = repo_mongo::ArticleDoc::find_by_ids(&article_ids).await;
    if find_r2.is_err() {
        warn!(
            "list_mine find_by_ids from mongo err: {}",
            find_r2.err().unwrap()
        );
        return global_response::new(
            global_response::ERROR_CODE_SERVER_ERROR,
            ArticleCreateVO::new(),
        );
    }

    let article_docs = find_r2.unwrap();

    let mut articles: Vec<Article> = vec![];
    for doc in article_docs {
        let mut article = Article::new();
        article.id = doc._id.unwrap().as_object_id().unwrap().to_string();
        article.title = doc.title;
        article.description = doc.description;
        article.summary = doc.summary;
        article.content = doc.content;
        article.create_time = doc.create_time;
        article.update_time = doc.update_time;

        articles.push(article);
    }

    global_response::new(
        global_response::ERROR_CODE_SUCCESS,
        new_page::<Vec<Article>>(
            page_param.size,
            page_param.current,
            article_page.total,
            articles,
        ),
    )
}
