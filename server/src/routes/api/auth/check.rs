use axum::Json;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::routes;

use crate::{middlewares::require_auth::UnauthorizedError, routes::RouteProtectionLevel};

use super::Route;

const PATH: &str = "/api/auth/check";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(check_system_auth),
        RouteProtectionLevel::SystemAuthenticated,
    )]
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
    path = PATH,
    responses(
        (status = OK, description = "Success", body = AuthCheckSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Other"
)]
async fn check_system_auth() -> Json<AuthCheckSuccess> {
    Json(AuthCheckSuccess { success: true })
}
