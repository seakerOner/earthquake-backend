### Brief Notes On Design Decisions

## Language & Framework

- Rust + Axum was chosen to ensure high performance and safety for asynchronous data handling.
- Axum provides direct integration with Tokio and is lightweight, ideal for RESTfil APIs.

## Database

- Table structure `earthquakes`:
    - `id` (PK)
    - `magnitude` (f32)
    - `place` (String)
    - `time` (i64)
    - `latitude`, `longitude` (f64)
    - `depth` (f64)
- `ON CONFLICT (id) DO NOTHING` is used to avoid duplicate inserts when handling batch events.

## Data Ingestion

- Data ingested via USGS API using the `Usgs` client, which dynamically builds URLs using enums and the `Display` trait.
- Supports filtering by time, magnitude and location.
- Inserts are done in **batches** (configurable with batchsize) to improve performance.

## API Design

- RESTful endpoints:
    - `GET /earthquakes` returns recent earthquakes with optional filters (`min_magnitude`, `max_magnitude`, `start_date`, 
    `start_date`, `end_date`, `limit`).
    - `GET /earthquakes/{id}` returns details of a specific earthquake.
- Uses Axum's `Query` and `Path` extractors for query parameters and path deserialization.
- Basic pagination with `limit` and ordering by `time DESC`

## Error Handling & Logging

- Network and database errors are handled via `Result` and logged using **tracing**.
- Logs are directed to the console (all levels) and to a file (only `INFO` and `ERROR`).

## OpenAPI Documentation

- **Utoipa** used to automatically generate OpenAPI documentation.
- Documents endpoints, schemas and tags, served via Swagger UI at `/docs`.

## Trade-offs & Limitations
- Unit tests only cover URL construction and JSON parsing.
- Database and network tests were not made due to setup complexity (requires creating a separate DB just for testing)
- `Clients` in a more mature project could have a Trait shared between all clients providing consistent function calls across clients,
even though it reduces flexibility for client-specific behaviors.
- In a more mature project, a graceful shutdown logic would make the system more robust. For example the background data_updater 
can easily hang.
- All API requests have direct access to the database, in high-traffic scenarios having some caching logic would help 
reduce loads.
- The data ingestion pipeline trusts `USGS` data, there aren't checks for malformed records, outliers or invalid coordinates.
- CORS layer doesn't have any restrictions, in a production scenario this would have to change.
