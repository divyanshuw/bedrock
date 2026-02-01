# Latest Changes - PostgreSQL Integration

## Detailed Changes

### `backend/services/ingestion/Cargo.toml`
- Added `dotenvy` dependency for managing environment variables.
- Updated `sqlx` dependency to include `postgres` and `runtime-tokio-rustls` features.

### `backend/services/ingestion/src/main.rs`
- Added `dotenvy::dotenv().ok()` to load environment variables from `.env` file.
- Initialized `PgPool` using `DATABASE_URL` from environment variables.
- Configured the database connection pool with a maximum of 5 connections.
- Passed the initialized `pool` to `ChainState`.

### `backend/services/ingestion/src/state.rs`
- Updated `ChainState` struct to include an optional `db_pool: Option<PgPool>`.
- Updated `Default` implementation for `ChainState` to initialize `db_pool` as `None`.
- Added `use sqlx::PgPool;`.

### `backend/services/ingestion/src/ingest.rs`
- Updated `run` function to extract the database pool from `ChainState` and pass it to `ingest_slot`.
- Updated `ingest_slot` function signature to accept `&PgPool`.
- Implemented logic in `ingest_slot` to:
    - Fetch block data using `rpc.get_block_by_slot`.
    - Create the `blocks` table if it doesn't exist.
    - Insert block data (slot, blockhash, previous_blockhash, parent_slot) into the `blocks` table.
- Added `use sqlx::PgPool;`.

### `backend/services/ingestion/src/rpc.rs`
- Added `get_block_by_slot` method to `RpcClient` to fetch a specific block by slot number.
- `get_block_by_slot` returns `Result<BlockData, IngestionError>`.

### `backend/services/ingestion/src/custom_err.rs`
- Added `BlockNotFound` variant to `IngestionError` enum to handle cases where a requested block is not found.

### `backend/services/ingestion/.env`
- Created a new `.env` file with a `DATABASE_URL` variable for database configuration.

### `.gitignore`
- Added `.env` to ignore list to prevent sensitive information from being committed.

## High-Level Summary
The `ingestion` service has been updated to integrate with a PostgreSQL database. The service now initializes a database connection pool on startup using credentials from a `.env` file. The core ingestion loop (`ingest.rs`) has been enhanced to persist fetched Solana block data into a `blocks` table in the database. Support for fetching specific blocks by slot was added to the RPC client, and error handling was improved to account for missing blocks.
