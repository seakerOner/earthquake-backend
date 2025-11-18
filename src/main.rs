use axum::{Router, routing::get};
use http::{
    Method,
    header::{AUTHORIZATION, CONTENT_TYPE, ORIGIN},
};
use tokio::{net::TcpListener, time::Duration};
use tower_http::cors::CorsLayer;
use tracing::{Level, error, info, instrument::WithSubscriber};
use tracing_subscriber::FmtSubscriber;
mod api;
use api::earthquakes;
mod clients;
use clients::{run_usgs_realtime_data_updater, usgs::*};

#[tokio::main]
async fn main() {
    let subscriber = FmtSubscriber::builder()
        .with_max_level(Level::TRACE)
        .finish();

    tracing::subscriber::set_global_default(subscriber).expect("Failed to setup tracing");

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET])
        .allow_headers([ORIGIN, CONTENT_TYPE, AUTHORIZATION])
        .max_age(Duration::from_secs(60 * 60));

    let app = Router::new()
        .route("/", get(root))
        .merge(earthquakes::router())
        .layer(cors);

    let listener = match TcpListener::bind("localhost:42069").await {
        Ok(l) => l,
        Err(e) => {
            error!("Server couldn't bind to port; Error: {}", e);
            panic!("Failed binding TcpListener to the given address: {}", e);
        }
    };

    // Background task getting new data to the database every 10 seconds
    tokio::spawn(run_usgs_realtime_data_updater(10));

    info!(r#"Serving at: http://localhost:42069"#);

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        error!("The server couldn't serve the service; Error: {}", e);
        panic!("Failed to serve the service: {}", e);
    }
}

async fn root() -> &'static str {
    "Hello Gravity"
}
