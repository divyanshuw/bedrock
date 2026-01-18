use crate::state::IngestionState;
use crate::{rpc::RpcClient, state::ChainState};

use tokio::time::{Duration, Interval, interval};
use tracing::{info, warn};

pub async fn run(rpc: RpcClient, mut state: ChainState) -> anyhow::Result<()> {
    info!(
        "starting ingestion loop from slot {}",
        state.ingest_state.last_ingested_block
    );

    let mut ticker: Interval = interval(Duration::from_secs(1));

    loop {
        ticker.tick().await;

        let latest_slot = match rpc.get_latest_slot().await {
            Ok(slot) => slot,
            Err(err) => {
                warn!("Failed to fetch the latest slot: {}", err);
                continue;
            }
        };

        if state.last_processed_slot >= latest_slot {
            continue;
        }

        let next_slot = state.last_processed_slot + 1;

        info!("ingesting slot {}", next_slot);

        if let Err(err) = ingest_slot(next_slot, &rpc).await {
            warn!("failed to ingest slot {}:{}", next_slot, err);
        }
    }
}

async fn ingest_slot(next_slot: u64, rpc: &RpcClient) -> Result<String, anyhow::Error> {
    Ok(String::from("hello"))
}
