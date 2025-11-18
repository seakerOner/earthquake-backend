# Earthquake Backend

A backend service for ingesting, storing, and exposing global earthquake data in real-time.  
Built with **Rust (Axum)**, **SQLx**, **PostgreSQL**, and **Utoipa** for OpenAPI documentation.

---

## Features

- Ingest earthquake data from USGS.
- Store data efficiently in PostgreSQL.
- RESTful API for:
  - `GET /earthquakes` → list recent earthquakes, optional filtering by magnitude and date.
  - `GET /earthquakes/{id}` → fetch details of a specific earthquake.
- OpenAPI documentation available via SwaggerUI.
- Logging to both console and a file.
- Background task for periodic data updates.

---

## Endpoints

| Endpoint                     | Method | Description                                                                                                        |
|------------------------------|--------|--------------------------------------------------------------------------------------------------------------------|
| `/earthquakes`               | GET    | List recent earthquakes with optional filters: `min_magnitude`, `max_magnitude`, `start_date`, `end_date`, `limit` |
| `/earthquakes/{id}`          | GET    | Fetch detailed earthquake information by ID                                                                        |
| `/docs`                      | GET    | SwaggerUI with OpenAPI docs                                                                                        |
| `/api-doc/openapi.json`      | GET    | Raw OpenAPI JSON                                                                                                   |

---

## Setup Instructions

### Prerequisites

- Docker & Docker Compose
- Cargo & sqlx-cli

### Steps

1. **Start Docker environment**  

   From the project root:  
   ```bash
   docker-compose up -d 
   ```

  - This will:
    - Start PostgreSQL
    - Install Rust if needed

2. Run database migrations (in the same directory where /migrations is)

    ```bash
    # To install the sqlx cli tool
    cargo install sqlx-cli --no-default-features --features postgres,rustls

    cargo sqlx migrate run
    ```

3. Run the application

    ```bash
    cargo run
    ```

  - The service will start at `http://localhost:42069`.

4. Access OpenAPI docs

    - SwaggerUI: `http://localhost:42069/docs`.
    - OpenAPI JSON: `http://localhost:42069/api-doc/openapi.json`

5. Log file

    - A `tracer.log` (or whatever your configured log file name is) will be created in the same directory where `cargo run` is executed.
    Console output will show all log levels; the file logs only `INFO` and `ERROR`

