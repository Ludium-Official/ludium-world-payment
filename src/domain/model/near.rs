use near_primitives::{types::AccountId, hash::CryptoHash, views::{
    ExecutionOutcomeWithIdView, FinalExecutionStatus,
}};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResult {
    pub status: FinalExecutionStatus,
    pub transaction_outcome: ExecutionOutcomeWithIdView,
    pub receipts_outcome: Vec<ExecutionOutcomeWithIdView>,
}


#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TransactionResultResponse {
    pub message: String,
    pub status: FinalExecutionStatus,
    pub receiver_id: AccountId,
    pub transaction_hash: CryptoHash,
}