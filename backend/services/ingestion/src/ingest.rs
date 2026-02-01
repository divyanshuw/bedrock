use crate::{rpc::RpcClient, state::ChainState};
use sqlx::PgPool;
use tokio::time::{interval, Duration, Interval};
use tracing::{info, warn};

pub async fn run(mut rpc: RpcClient, state: ChainState) -> anyhow::Result<()> {
    info!(
        "starting ingestion loop from slot {}",
        state.ingest_state.last_processed_slot
    );

    let mut ticker: Interval = interval(Duration::from_secs(1));
    let pool = state.db_pool.as_ref().unwrap();

    loop {
        ticker.tick().await;

        let _latest_block = match rpc.get_block().await {
            Ok(block) => block,
            Err(err) => {
                warn!("Failed to fetch the latest block: {}", err);
                continue;
            }
        };

        let next_slot = state.ingest_state.last_processed_slot + 1;

        info!("ingesting slot {}", next_slot);

        if let Err(err) = ingest_slot(next_slot, &rpc, pool).await {
            warn!("failed to ingest slot {}:{}", next_slot, err);
        }
    }
}

async fn ingest_slot(slot: u64, rpc: &RpcClient, pool: &PgPool) -> anyhow::Result<()> {
    let block= rpc.get_block_by_slot(slot).await?;

    // WHY DO WE NEED THIS?
    sqlx::query(
        "CREATE TABLE IF NOT EXISTS blocks (
            slot BIGINT PRIMARY KEY,
            blockhash TEXT NOT NULL,
            previous_blockhash TEXT NOT NULL,
            parent BIGINT NOT NULL,
            ingested_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
        )",
    )
    .execute(pool)
    .await?;

    let result = sqlx::query(
        "INSERT INTO blocks (slot, blockhash, previous_blockhash, parent)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(slot as i64)
    .bind(block.blockhash)
    .bind(block.previous_blockhash)
    .bind(block.parent_slot as i64)
    .execute(pool)
    .await?;

    info!("inserted {} rows for slot {}", result.rows_affected(), slot);

    Ok(())
}
