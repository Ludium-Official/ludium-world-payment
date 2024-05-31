use near_primitives::{types::AccountId, hash::CryptoHash, views::{
    ExecutionOutcomeWithIdView, FinalExecutionStatus,
}};
use serde::{Deserialize, Serialize};


pub enum TransferActionType {
    Native { user_address: String, amount_in_smallest_unit: u128 },
    FtTransfer { ft_contract_id: AccountId, user_address: String, amount_in_smallest_unit: u128 },
}


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
    pub has_errors: bool,
    pub error_details: Vec<String>,
}