use crate::structs::global_response;
use crate::structs::Claims;
use crate::structs::{
    Article, ArticleCreateAO, ArticleCreateVO, ArticleListMineAO, ArticleUpdateAO, ArticleUpdateVO,
    PageParam,
};
use axum::extract::Json;
use axum::http::HeaderMap;
use serde_json::Value;
use tracing::info;

#[tracing::instrument]
pub async fn create_article_handler(
    headers: HeaderMap,
    claims: Claims,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let request_id = super::get_trace_id_from_header(&headers);
    info!("create_article_handler enter, trace_id: {:?}", request_id);

    let req: ArticleCreateAO = serde_json::from_value(payload).unwrap();
    if req.title.is_empty() {
        return global_response::new(
            global_response::ERROR_CODE_PARAM_INVALID,
            ArticleCreateVO::new(),
        );
    }

    crate::service::article::create(&claims.user_info, &req).await
}

#[tracing::instrument]
pub async fn update_article_handler(
    headers: HeaderMap,
    claims: Claims,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let request_id = super::get_trace_id_from_header(&headers);
    info!("update_article_handler enter, trace_id: {:?}", request_id);

    let req: ArticleUpdateAO = serde_json::from_value(payload).unwrap();
    if req.id.is_empty() || req.title.is_empty() {
        return global_response::new(
            global_response::ERROR_CODE_PARAM_INVALID,
            ArticleUpdateVO::new(),
        );
    }

    crate::service::article::update(&claims.user_info, &req).await
}

#[tracing::instrument]
pub async fn search_article_handler(
    headers: HeaderMap,
    _claims: Claims,
    Json(_payload): Json<Value>,
) -> Json<Value> {
    let request_id = super::get_trace_id_from_header(&headers);
    info!("search_article_handler enter, trace_id: {:?}", request_id);

    let article_items: Vec<Article> = vec![];

    crate::structs::global_response::new(
        crate::structs::global_response::ERROR_CODE_SUCCESS,
        crate::structs::new_page::<Vec<Article>>(0, 0, 0, article_items),
    )
}

#[tracing::instrument]
pub async fn list_mine_article_handler(
    headers: HeaderMap,
    claims: Claims,
    Json(payload): Json<Value>,
) -> Json<Value> {
    let request_id = super::get_trace_id_from_header(&headers);
    info!(
        "list_mine_article_handler enter, trace_id: {:?}",
        request_id
    );

    let mut req: ArticleListMineAO = serde_json::from_value(payload).unwrap();
    if req.page_param.is_none() {
        req.page_param = Some(PageParam {
            size: 10,
            current: 1,
        });
    }

    crate::service::article::list_mine(&claims.user_info, &req).await
}
