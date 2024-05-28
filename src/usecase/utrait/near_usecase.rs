use std::sync::Arc;

use axum::async_trait;
use crate::{domain::model::near::TransactionResultResponse, port::output::rpc_client::RpcClient};
use crate::usecase::error::Result;

#[async_trait]
pub trait NearUsecase {
    async fn relay(&self, near_rpc_manager: &Arc<dyn RpcClient>, data: Vec<u8>) -> Result<TransactionResultResponse>; 

}
