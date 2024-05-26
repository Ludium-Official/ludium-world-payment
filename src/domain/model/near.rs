use near_primitives::views::{
    ExecutionOutcomeWithIdView, FinalExecutionStatus,
};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResult {
    pub status: FinalExecutionStatus,
    pub transaction_outcome: ExecutionOutcomeWithIdView,
    pub receipts_outcome: Vec<ExecutionOutcomeWithIdView>,
}


#[derive(Debug, Serialize, Deserialize)]
pub struct TransactionResultResponse {
    pub message: String,
    pub status: FinalExecutionStatus,
    pub transaction_outcome: ExecutionOutcomeWithIdView,
    pub receipts_outcome: Vec<ExecutionOutcomeWithIdView>,
}