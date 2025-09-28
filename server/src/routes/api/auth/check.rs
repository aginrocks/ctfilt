use axum::{Json, middleware};
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    middlewares::require_auth::{UnauthorizedError, require_system_auth},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(check_system_auth))
        .layer(middleware::from_fn(require_system_auth))
}

#[derive(Serialize, ToSchema)]
pub struct AuthCheckSuccess {
    pub success: bool,
}

/// Check system auth
///
/// Checks if your API key is valid
///
/// This endpoint is only avaliable to users authenticating with a system-wide API key
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = AuthCheckSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Auth"
)]
async fn check_system_auth() -> Json<AuthCheckSuccess> {
    Json(AuthCheckSuccess { success: true })
}
