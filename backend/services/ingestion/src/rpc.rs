use crate::custom_err::IngestionError;
use reqwest::Client;
use serde::{Deserialize, Serialize};
use serde_json::{self};
use std::collections::VecDeque;
// Payload structs

#[derive(Debug, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct BlockParams {
    commitment: String,
    encoding: String,
    transaction_details: String,
    max_supported_transaction_version: u64,
    rewards: bool,
}

impl Default for BlockParams {
    fn default() -> Self {
        Self {
            commitment: String::from("finalized"),
            encoding: String::from("encoding"),
            transaction_details: String::from("full"),
            max_supported_transaction_version: 0,
            rewards: false,
        }
    }
}

#[derive(Debug, Serialize)]
pub struct BlockPayload {
    jsonrpc: String,
    id: usize,
    method: String,
    params: (u64, BlockParams),
}

impl BlockPayload {
    pub fn new(slot: u64, params: BlockParams) -> Self {
        BlockPayload {
            jsonrpc: String::from("2.0"),
            id: 1,
            method: String::from("getBlock"),
            params: (slot, params),
        }
    }
}

// slot and block deserializer

#[derive(Deserialize)]
pub struct Slot {
    jsonrpc: String,
    result: u64,
    id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Block {
    jsonrpc: String,
    result: Option<BlockData>,
    id: u64,
}

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BlockData {
    pub block_height: Option<u64>,
    pub block_time: Option<i64>,
    pub blockhash: String,
    pub parent_slot: u64,
    pub previous_blockhash: String,
    pub transactions: VecDeque<Transaction>,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Transaction {
    pub meta: Option<serde_json::Value>,
    pub transaction: TransactionData,
}

#[derive(serde::Deserialize, Debug)]
pub struct TransactionData {
    pub message: Message,
}

#[derive(serde::Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Message {
    pub recent_blockhash: String,
}

// Rpc client struct

pub struct RpcClient {
    client: Client,
    address: String,
}

impl Default for RpcClient {
    fn default() -> Self {
        Self::new("https://api.devnet.solana.com")
    }
}

impl RpcClient {
    pub fn new(address: impl Into<String>) -> Self {
        Self {
            client: Client::new(),
            address: address.into(),
        }
    }
    pub async fn get_latest_slot(&self) -> Result<u64, IngestionError> {
        let payload = serde_json::json!({
            "jsonrpc": "2.0",
            "id": 1,
            "method": "getSlot",
            "params": [{
                "commitment": "finalized"
            }]
        });

        let res = self
            .client
            .post(&self.address)
            .json(&payload)
            .send()
            .await
            .map_err(|e| IngestionError::NetworkError(e.to_string()))?;

        let slot_res: Slot = res
            .json()
            .await
            .map_err(|e| IngestionError::JsonParseError(e.to_string()))?;
        Ok(slot_res.result)
    }

    pub async fn get_block(&mut self) -> Result<Block, IngestionError> {
        // Alwayas get the latest slot

        if let Ok(value) = self.get_latest_slot().await {
            let params = BlockParams::default();
            let payload = BlockPayload::new(value, params);

            let res = self
                .client
                .post(&self.address)
                .json(&payload)
                .send()
                .await
                .map_err(|e| IngestionError::BlockResponseError(e.to_string()))?;

            let block_response: Block = res
                .json()
                .await
                .map_err(|e| IngestionError::JsonParseError(e.to_string()))?;
            Ok(block_response)
        } else {
            Err(IngestionError::RpcError(String::from(
                "Failed to get the latest slot",
            )))
        }
    }

    pub async fn get_block_by_slot(&self, slot: u64) -> Result<BlockData, IngestionError> {
        let params = BlockParams::default();
        let payload = BlockPayload::new(slot, params);
        let res = self
            .client
            .post(&self.address)
            .json(&payload)
            .send()
            .await
            .map_err(|e| IngestionError::BlockResponseError(e.to_string()))?;

        let block_response: Block = res
            .json()
            .await
            .map_err(|e| IngestionError::JsonParseError(e.to_string()))?;

        if let Some(block_data) = block_response.result {
            Ok(block_data)
        } else {
            Err(IngestionError::BlockNotFound)
        }
    }
}
