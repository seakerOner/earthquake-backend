pub mod usgs;

use serde_json::Value;
use std::time::Duration;
use tracing::{error, info};

use usgs::{ExtParameters, OrderingOptions, Usgs};

pub async fn run_usgs_realtime_data_updater(sleep_time_secs: u64) {
    let usgs = Usgs::new();

    loop {
        match usgs
            .query(
                usgs::QueryFormats::GeoJSON,
                vec![
                    ExtParameters::SetEarthquakeEvent(),
                    ExtParameters::OrderBy(OrderingOptions::Time),
                ],
            )
            .await
        {
            Ok(s) => {
                //TODO: SEND VALUES TO DB
                println!("{:#?}", s);
            }
            Err(e) => error!("Error on background USGS query data updater; Error: {}", e),
        }

        tokio::time::sleep(Duration::from_secs(sleep_time_secs)).await;
    }
}
