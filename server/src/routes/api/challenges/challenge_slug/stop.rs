use axum::Json;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::UnauthorizedError,
    routes::{
        RouteProtectionLevel,
        api::{NotFoundError, challenges::challenge_slug::KubernetesActionResult},
    },
};

use super::Route;

const PATH: &str = "/api/challenges/{challenge_slug}/stop";

pub fn routes() -> Vec<Route> {
    [vec![(
        routes!(stop_challenge),
        RouteProtectionLevel::Authenticated,
    )]]
    .concat()
}

/// Stop a challenge
#[utoipa::path(
    method(post),
    path = PATH,
    params(
        ("challenge_slug" = String, Path, description = "Challenge slug"),
    ),
    responses(
        (status = OK, description = "Success", body = KubernetesActionResult, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Challenge not found", body = NotFoundError, content_type = "application/json")
    ),
    tag = "Challenge"
)]
async fn stop_challenge() -> AxumResult<Json<KubernetesActionResult>> {
    todo!()
}
