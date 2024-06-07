use utoipa::openapi::security::{ApiKey, ApiKeyValue, SecurityScheme};
use utoipa::{Modify, OpenApi};

use crate::adapter::input::web::_dev_routes_login::LoginPayload;
use crate::adapter::input::web::middleware::response::ErrorResponse;
use crate::domain::model::coin_network::CoinNetworkDetailsResponse;
use crate::domain::model::network::NetworkResponse;
use crate::domain::model::coin::CoinResponse;
use crate::domain::model::reward_claim::{CombinedRewardClaimResponse, NewRewardClaimPayload};
use crate::adapter::input::web::routes_hello::__path_hello;
use crate::adapter::input::web::routes_coin_network::__path_list_coin_networks;
use crate::adapter::input::web::routes_reward_claim::{__path_create_reward_claim, __path_list_me_reward_claim};
use crate::domain::model::reward_claim_detail::RewardClaimDetailResponse;

struct SecurityAddon;

impl Modify for SecurityAddon {
    fn modify(&self, openapi: &mut utoipa::openapi::OpenApi) {
        if let Some(components) = openapi.components.as_mut() {
            components.add_security_scheme(
                "access_token",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("access_token"))),
            );
            components.add_security_scheme(
                "ggl_id",
                SecurityScheme::ApiKey(ApiKey::Cookie(ApiKeyValue::new("ggl_id"))),
            );
            components.add_security_scheme(
                "x-user-right",
                SecurityScheme::ApiKey(ApiKey::Header(ApiKeyValue::new("x-user-right"))),
            );
        }
    }
}


#[derive(OpenApi)]
#[openapi(
    modifiers(&SecurityAddon),
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
            LoginPayload,
            NewRewardClaimPayload,
            CoinNetworkDetailsResponse, CoinResponse, NetworkResponse,
            CombinedRewardClaimResponse, RewardClaimDetailResponse,
            ErrorResponse
        )
    ),
    security(
        ("access_token" = []),
        ("ggl_id" = []),
        ("x-user-right" = [])
    )
)]
pub struct ApiDoc;