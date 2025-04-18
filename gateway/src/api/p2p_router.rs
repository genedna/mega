use std::collections::HashMap;

use crate::api::MegaApiServiceState;
use axum::{
    extract::{Query, State},
    http::StatusCode,
    routing::get,
    Json, Router,
};
use common::model::CommonResult;

pub fn routers() -> Router<MegaApiServiceState> {
    Router::new()
        .route("/p2p/repo_fork", get(repo_fork))
        .route("/p2p/repo_share", get(repo_share))
        .route("/p2p/peer_id", get(peer_id))
}

async fn repo_fork(
    Query(query): Query<HashMap<String, String>>,
    state: State<MegaApiServiceState>,
) -> Result<Json<CommonResult<String>>, (StatusCode, String)> {
    let identifier = match query.get("identifier") {
        Some(i) => i,
        None => {
            return Err((
                StatusCode::BAD_REQUEST,
                String::from("Identifier not provide\n"),
            ));
        }
    };

    let res =
        gemini::p2p::client::repo_clone(state.inner.context.clone(), identifier.to_string()).await;
    let res = match res {
        Ok(_) => CommonResult::success(Some("ok".to_string())),
        Err(err) => CommonResult::failed(&err.to_string()),
    };

    Ok(Json(res))
}

async fn repo_share(
    Query(query): Query<HashMap<String, String>>,
    state: State<MegaApiServiceState>,
) -> Result<Json<CommonResult<String>>, (StatusCode, String)> {
    let path = match query.get("path") {
        Some(i) => i,
        None => {
            return Err((StatusCode::BAD_REQUEST, String::from("path not provide\n")));
        }
    };

    let res = gemini::p2p::client::repo_share(state.inner.context.clone(), path.clone()).await;
    let res = match res {
        Ok(s) => CommonResult::success(Some(s.to_string())),
        Err(err) => CommonResult::failed(&err.to_string()),
    };

    Ok(Json(res))
}

async fn peer_id(
    Query(_query): Query<HashMap<String, String>>,
    _state: State<MegaApiServiceState>,
) -> Result<Json<CommonResult<String>>, (StatusCode, String)> {
    let peer_id = vault::get_peerid().await;
    Ok(Json(CommonResult::success(Some(peer_id))))
}
