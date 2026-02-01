use crate::{rpc::RpcClient, state::ChainState};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions};
use std::env;

mod custom_err;
mod ingest;
mod rpc;
mod state;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let rpc: RpcClient = RpcClient::default();
    let mut state: ChainState = ChainState::default();
    let database_url: String = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let pool:Pool<Postgres> = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create pool.");

    state.db_pool = Some(pool);

    let result = ingest::run(rpc, state).await.unwrap();

    println!("ingestion procedure result! {:?}", result);
}
