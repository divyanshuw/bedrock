use thiserror::Error;

#[derive(Debug, Error)]
pub enum IngestionError {
    #[error("network error while calling Solana RPC")]
    NetworkError(String),

    #[error("failed to parse JSON response")]
    JsonParseError(String),

    #[error("failed to recieve block info")]
    BlockResponseError(String),

    #[error("rpc response error")]
    RpcError,
}
