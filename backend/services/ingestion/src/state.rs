use sqlx::PgPool;
pub use crate::rpc::BlockData;
pub use std::collections::VecDeque;

pub enum ReorgResult {
    NoReorg,
    ReorgDetected {
        expexted_parent: String,
        actual_parent: String,
    },
}

pub struct IngestionState {
    pub chain_id: String,
    pub last_ingested_block: u64,
    pub last_finalized_block: u64,
    pub last_blockhash: String,
    pub last_processed_slot: u64,
}

#[derive(Clone, Debug)]
pub struct ReorgState {
    pub recent_blocks: VecDeque<(u64, String)>,
    pub max_depth: usize,
}

pub struct ChainState {
    pub ingest_state: IngestionState,
    pub reorg_state: ReorgState,
    pub db_pool: Option<PgPool>,
}

impl Default for ChainState {
    fn default() -> Self {
        ChainState {
            ingest_state: IngestionState {
                chain_id: String::from("solana-devnet"),
                last_ingested_block: 0,
                last_finalized_block: 0,
                last_blockhash: String::new(),
                last_processed_slot: 0,
            },

            reorg_state: ReorgState {
                recent_blocks: VecDeque::new(),
                max_depth: 100,
            },
            db_pool: None,
        }
    }
}

impl ChainState {
    pub fn advance_block(&mut self, block_hash: String, block_number: u64) {
        self.ingest_state.last_ingested_block = block_number;

        self.reorg_state
            .recent_blocks
            .push_back((block_number, block_hash));

        if self.reorg_state.recent_blocks.len() > self.reorg_state.max_depth {
            self.reorg_state.recent_blocks.pop_front();
        }
    }

    pub fn finalize_block(&mut self, block_number: u64) {
        self.ingest_state.last_finalized_block = block_number;
    }

    pub async fn detect_reorg(
        current_block_data: BlockData,
        previous_state: ChainState,
    ) -> ReorgResult {
        if previous_state.ingest_state.last_blockhash != current_block_data.previous_blockhash {
            return ReorgResult::ReorgDetected {
                expexted_parent: previous_state.ingest_state.last_blockhash,
                actual_parent: current_block_data.previous_blockhash,
            };
        } else {
            ReorgResult::NoReorg
        }
    }
}
