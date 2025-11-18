use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;
use std::fmt::Display;
use tracing::error;

pub struct Usgs {
    base_url: String,
}

#[allow(dead_code)]
pub enum UsgsMethods {
    KnownParameterValues,
    AppWADL,
    Catalogs,
    Contributors,
    Count,
    Query,
    Version,
}

impl Display for UsgsMethods {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::KnownParameterValues => write!(f, "application.json"),
            Self::AppWADL => write!(f, "application.wadl"),
            Self::Catalogs => write!(f, "catalogs"),
            Self::Contributors => write!(f, "contributors"),
            Self::Count => write!(f, "contributors"),
            Self::Query => write!(f, ""),
            Self::Version => write!(f, "version"),
        }
    }
}

#[allow(dead_code)]
pub enum QueryFormats {
    GeoJSON,
    Kml,
    QuakeMl,
    Text,
}

impl Display for QueryFormats {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::GeoJSON => write!(f, "format=geojson"),
            Self::Kml => write!(f, "format=kml"),
            Self::QuakeMl => write!(f, "format=xml"), // Alias
            Self::Text => write!(f, "format=text"),
        }
    }
}

#[allow(dead_code)]
pub enum Time {
    StartTime(String),
    EndTime(String),
    UpdatedAfter(String),
}

impl Display for Time {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::StartTime(s) => write!(f, "starttime={}", s),
            Self::EndTime(s) => write!(f, "endtime={}", s),
            Self::UpdatedAfter(s) => write!(f, "updatedafter={}", s),
        }
    }
}

#[allow(dead_code)]
pub enum Rectangle {
    MinLatitude(i8),
    MinLongitude(i16),
    MaxLatitude(i8),
    MaxLongitude(i16),
}

impl Display for Rectangle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::MinLatitude(n) => write!(f, "minlatitude={}", n.to_string()),
            Self::MinLongitude(n) => write!(f, "minlongitude={}", n.to_string()),
            Self::MaxLatitude(n) => write!(f, "maxlatitude={}", n.to_string()),
            Self::MaxLongitude(n) => write!(f, "maxlongitude={}", n.to_string()),
        }
    }
}

#[allow(dead_code)]
pub enum Circle {
    Latitude(i8),
    Longitude(i8),
    MaxRadius(u8),
    MaxRadiusKm(f32),
}

impl Display for Circle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Latitude(n) => write!(f, "latitude={}", n.to_string()),
            Self::Longitude(n) => write!(f, "longitude={}", n.to_string()),
            Self::MaxRadius(n) => write!(f, "maxradius={}", n.to_string()),
            Self::MaxRadiusKm(n) => write!(f, "maxradiuskm={}", n.to_string()),
        }
    }
}

#[allow(dead_code)]
pub enum ExtParameters {
    OrderBy(OrderingOptions),
    AlertLevel(AlertLevelOptions),
    JsonError(bool),
    SetEarthquakeEvent(),
}

impl Display for ExtParameters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OrderBy(o) => write!(f, "orderby={}", o),
            Self::AlertLevel(l) => write!(f, "alertlevel={}", l),
            Self::JsonError(b) => write!(f, "jsonerror={}", b),
            Self::SetEarthquakeEvent() => write!(f, "eventtype=earthquake"),
        }
    }
}

#[allow(dead_code)]
pub enum OrderingOptions {
    Time,
    TimeAsc,
    Magnitude,
    MagnitudeAsc,
}

impl Display for OrderingOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Time => write!(f, "time"),
            Self::TimeAsc => write!(f, "time-asc"),
            Self::Magnitude => write!(f, "magnitude"),
            Self::MagnitudeAsc => write!(f, "magnitude-asc"),
        }
    }
}

#[allow(dead_code)]
pub enum AlertLevelOptions {
    Green,
    Yellow,
    Orange,
    Red,
}

impl Display for AlertLevelOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Green => write!(f, "green"),
            Self::Yellow => write!(f, "yellow"),
            Self::Orange => write!(f, "orange"),
            Self::Red => write!(f, "red"),
        }
    }
}
impl Usgs {
    pub fn new() -> Self {
        Usgs {
            base_url: String::from("https://earthquake.usgs.gov/fdsnws/event/1/"),
        }
    }

    pub async fn query(
        &self,
        format: QueryFormats,
        params: Vec<impl Display>,
    ) -> Result<Vec<Earthquake>, ()> {
        let mut url = format!("{}query?{}", self.base_url, format);

        for p in params {
            url.push('&');
            url.push_str(&p.to_string());
        }

        let resp = match reqwest::get(url).await {
            Ok(r) => match r.text().await {
                Ok(m) => m,
                Err(e) => {
                    error!("Error getting response from API; Error: {}", e);
                    return Err(());
                }
            },
            Err(e) => {
                error!("Error fetching API; Error: {}", e);
                return Err(());
            }
        };

        let json: UsgsResponse = match serde_json::from_str(&resp) {
            Ok(j) => j,
            Err(e) => {
                error!("Error parsing string to Json on API response; Error: {}", e);
                return Err(());
            }
        };

        Ok(json.into_earthquakes())
    }
}

#[derive(Debug, Deserialize)]
pub struct UsgsResponse {
    pub features: Vec<UsgsFeature>,
}

impl UsgsResponse {
    pub fn into_earthquakes(self) -> Vec<Earthquake> {
        self.features
            .into_iter()
            .filter_map(|f| {
                let mag = f.properties.mag?;
                let place = f.properties.place?;
                let time = f.properties.time?;

                let lon = f.geometry.coordinates[0];
                let lat = f.geometry.coordinates[1];
                let depth = f.geometry.coordinates[2];

                Some(Earthquake {
                    id: f.id,
                    magnitude: mag,
                    place,
                    time,
                    latitude: lat,
                    longitude: lon,
                    depth,
                })
            })
            .collect()
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsgsFeature {
    pub id: String,
    pub properties: UsgsProperties,
    pub geometry: UsgsGeometry,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsgsProperties {
    pub mag: Option<f32>,
    pub place: Option<String>,
    pub time: Option<i64>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct UsgsGeometry {
    pub coordinates: [f64; 3],
}

#[derive(Debug, Deserialize, Serialize, FromRow)]
pub struct Earthquake {
    pub id: String,
    pub magnitude: f32,
    pub place: String,
    pub time: i64,
    pub latitude: f64,
    pub longitude: f64,
    pub depth: f64,
}
