use axum::{Router, routing::get};

pub fn router() -> Router {
    Router::new()
        .route("/earthquakes", get(earthquakes))
        .route("/earthquakes/{id}", get(earthquakes_by_id))
}

async fn earthquakes() {}

async fn earthquakes_by_id() {}
