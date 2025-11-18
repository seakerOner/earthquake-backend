use sqlx::{Pool, Postgres, postgres::PgPoolOptions, query, query_builder::QueryBuilder};
use std::env;
use tracing::error;
use tracing::info;

use crate::clients::usgs::Earthquake;

pub type DbPool = Pool<Postgres>;

pub async fn init_db() -> Result<DbPool, sqlx::Error> {
    dotenvy::from_path(std::path::Path::new(&format!(
        "{}/.env",
        env!("CARGO_MANIFEST_DIR")
    )))
    .ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL should be defined in the .env file");

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(std::time::Duration::from_secs(5))
        .connect(&database_url)
        .await?;

    info!("Connected to PostgreSQL");
    Ok(pool)
}

pub async fn db_query_list_earthquakes(
    earthquakes: Vec<Earthquake>,
    batchsize: usize,
    db: &DbPool,
) {
    if earthquakes.is_empty() {
        return;
    }

    for chunk in earthquakes.chunks(batchsize) {
        let mut builder = QueryBuilder::new(
            "INSERT INTO earthquakes (id, magnitude, place, time, latitude, longitude, depth)
            VALUES ",
        );

        let mut first = true;

        for eq in chunk {
            if !first {
                builder.push(", ");
            }
            builder
                .push("(")
                .push_bind(&eq.id)
                .push(", ")
                .push_bind(&eq.magnitude)
                .push(", ")
                .push_bind(&eq.place)
                .push(", ")
                .push_bind(&eq.time)
                .push(", ")
                .push_bind(&eq.latitude)
                .push(", ")
                .push_bind(&eq.longitude)
                .push(", ")
                .push_bind(&eq.depth)
                .push(")");
            first = false;
        }
        builder.push("ON CONFLICT (id) DO NOTHING");

        let res = builder.build().execute(db).await;
        if let Err(e) = res {
            error!("Database query fail; Error: {}", e);
        }
    }
}
