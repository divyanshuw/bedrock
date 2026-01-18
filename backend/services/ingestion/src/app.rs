use crate::ingest;
use crate::rpc::RpcClient;
use crate::state::IngestionState;

pub fn run() -> anyhow::Result<()> {
    let rpc = RpcClient::default();
    let state = IngestionState;
    ingest::run(rpc, state).await;
}