pub mod usgs;

use super::db::{DbPool, db_query_list_earthquakes};
use std::time::Duration;
use tracing::error;
use usgs::{ExtParameters, OrderingOptions, QueryFormats, Usgs};

pub async fn run_usgs_realtime_data_updater(sleep_time_secs: u64, db: DbPool) {
    let usgs = Usgs::new();

    loop {
        match usgs
            .query(
                QueryFormats::GeoJSON,
                vec![
                    ExtParameters::SetEarthquakeEvent(),
                    ExtParameters::OrderBy(OrderingOptions::Time),
                ],
            )
            .await
        {
            Ok(s) => db_query_list_earthquakes(s, 1000, &db).await,
            Err(_) => error!("Error on background USGS query data updater"),
        }

        tokio::time::sleep(Duration::from_secs(sleep_time_secs)).await;
    }
}
