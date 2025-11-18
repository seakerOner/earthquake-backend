use axum::{
    Extension, Json, Router,
    extract::{Path, Query},
    response::IntoResponse,
    routing::get,
};
use http::StatusCode;
use serde::Deserialize;

use crate::db::{DbPool, db_query_get_earthquakes_by_id, db_query_get_earthquakes_filtered};

pub fn router() -> Router {
    Router::new()
        .route("/earthquakes", get(earthquakes))
        .route("/earthquakes/:id", get(earthquakes_by_id))
}

#[derive(Debug, Deserialize)]
pub struct EarthquakeQuery {
    pub min_magnitude: Option<f32>,
    pub max_magnitude: Option<f32>,
    pub start_date: Option<i64>,
    pub end_date: Option<i64>,
    pub limit: Option<i64>,
}

async fn earthquakes(
    Extension(db): Extension<DbPool>,
    Query(params): Query<EarthquakeQuery>,
) -> impl IntoResponse {
    match db_query_get_earthquakes_filtered(&db, params).await {
        Ok(e) => (StatusCode::OK, Json(serde_json::json!(e))),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error": "failed_to_fetch_recent_earthquakes"})),
        ),
    }
}

async fn earthquakes_by_id(
    Extension(db): Extension<DbPool>,
    Path(id): Path<String>,
) -> impl IntoResponse {
    match db_query_get_earthquakes_by_id(&db, id).await {
        Ok(e) => (StatusCode::OK, Json(serde_json::json!(e))),
        Err(sqlx::Error::RowNotFound) => (
            StatusCode::NOT_FOUND,
            Json(serde_json::json!({ "error": "earthquake_not_found" })),
        ),
        Err(_) => (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({ "error" : "internal_server_error" })),
        ),
    }
}
