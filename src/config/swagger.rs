use utoipa::OpenApi;

use crate::adapter::input::web::middleware::response::ErrorResponse;
use crate::domain::model::coin_network::CoinNetworkDetailsResponse;
use crate::domain::model::network::NetworkResponse;
use crate::domain::model::coin::CoinResponse;
use crate::domain::model::reward_claim::{CombinedRewardClaimResponse, NewRewardClaimPayload};
use crate::adapter::input::web::routes_hello::__path_hello;
use crate::adapter::input::web::routes_coin_network::__path_list_coin_networks;
use crate::adapter::input::web::routes_reward_claim::{__path_create_reward_claim, __path_list_me_reward_claim};
use crate::domain::model::reward_claim_detail::RewardClaimDetailResponse;


#[derive(OpenApi)]
#[openapi(
    info(
        title = "Ludium Payment",
        description = "APIs for payment relayer system",
    ),
    paths(
        hello,
        list_coin_networks,
        create_reward_claim,
        list_me_reward_claim
        
    ),
    components(
        schemas(
            NewRewardClaimPayload,
            CoinNetworkDetailsResponse, CoinResponse, NetworkResponse,
            CombinedRewardClaimResponse, RewardClaimDetailResponse,
            ErrorResponse
        )
    ),
)]
pub struct ApiDoc;