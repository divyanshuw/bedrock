pub use std::collections::VecDeque;

pub struct IngestionState {
    pub chain_id: String,

    pub last_ingested_block: u64,
    pub last_finalized_block: u64,
}

#[derive(Clone, Debug)]
pub struct ReorgState {
    pub recent_blocks: VecDeque<(u64, String)>,
    pub max_depth: usize,
}

pub struct ChainState {
    pub ingest_state: IngestionState,
    pub reorg_state: ReorgState,
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

    pub fn detect_reorg(&self, new_hash: &String, block_number: u64) -> Option<u64> {
        for (num, hash) in self.reorg_state.recent_blocks.iter() {
            if *num == block_number && hash != new_hash {
                return Some(*num);
            }
        }
        None
    }
}
