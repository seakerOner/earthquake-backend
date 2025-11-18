use axum::{Router, routing::get};
use http::{
    Method,
    header::{AUTHORIZATION, CONTENT_TYPE, ORIGIN},
};
use tokio::{net::TcpListener, time::Duration};
use tower_http::{add_extension::AddExtensionLayer, cors::CorsLayer};
use tracing::{Level, error, info};
use tracing_subscriber::{
    EnvFilter, Layer,
    fmt::{self},
    layer::SubscriberExt,
};
mod api;
use api::earthquakes;
mod clients;
use clients::run_usgs_realtime_data_updater;
mod db;
use db::{DbPool, init_db};
use std::fs::OpenOptions;
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

#[tokio::main]
async fn main() {
    let log_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("earthquake.log")
        .expect("Cannot open log file");

    let file_layer = fmt::Layer::new()
        .with_writer(log_file)
        .with_filter(EnvFilter::from_default_env().add_directive(Level::INFO.into()));

    let console_layer = fmt::Layer::new()
        .with_filter(EnvFilter::from_default_env().add_directive(Level::TRACE.into()));

    let subscriber = tracing_subscriber::registry()
        .with(file_layer)
        .with(console_layer);

    tracing::subscriber::set_global_default(subscriber).expect("Failed to setup tracing");

    let database: DbPool = init_db().await.unwrap();

    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET])
        .allow_headers([ORIGIN, CONTENT_TYPE, AUTHORIZATION])
        .max_age(Duration::from_secs(60 * 60));

    let db_for_updater = database.clone();

    let app = Router::new()
        .route("/", get(root))
        .merge(earthquakes::router())
        .merge(SwaggerUi::new("/docs").url("/api-doc/openapi.json", ApiDoc::openapi()))
        .layer(AddExtensionLayer::new(database))
        .layer(cors);

    // Background task getting new data to the database every 10 seconds
    tokio::spawn(run_usgs_realtime_data_updater(10, db_for_updater));

    let listener = match TcpListener::bind("localhost:42069").await {
        Ok(l) => l,
        Err(e) => {
            error!("Server couldn't bind to port; Error: {}", e);
            panic!("Failed binding TcpListener to the given address: {}", e);
        }
    };

    info!(r#"Serving at: http://localhost:42069"#);
    info!(r#"SwaggerUI OpenAPI docs: http://localhost:42069/docs"#);
    info!(r#"OpenAPI JSON: http://localhost:42069/api-doc/openapi.json"#);

    if let Err(e) = axum::serve(listener, app.into_make_service()).await {
        error!("The server couldn't serve the service; Error: {}", e);
        panic!("Failed to serve the service: {}", e);
    }
}

async fn root() -> &'static str {
    "Hello Gravity"
}

#[derive(OpenApi)]
#[openapi(
    paths(
        crate::api::earthquakes::earthquakes,
        crate::api::earthquakes::earthquakes_by_id
    ),
    components(
        schemas(crate::clients::usgs::Earthquake, crate::api::earthquakes::EarthquakeQuery)
    ),
    tags(
        (name = "Earthquakes", description = "Earthquake API endpoints")
    )
)]
pub struct ApiDoc;
