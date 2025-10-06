use axum::{Extension, Json, middleware};
use chrono::Duration;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    database::Challenge,
    middlewares::require_auth::{UnauthorizedError, UserId, require_auth},
    routes::api::{
        GenericError, NotFoundError, challenges::challenge_slug::KubernetesActionResult,
    },
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(extend_challenge))
        .layer(middleware::from_fn(require_auth))
}

/// Extend a challenge
///
/// This endpoint adds more time to a challenge instance.
#[utoipa::path(
    method(post),
    path = "/",
    params(
        ("challenge_slug" = String, Path, description = "Challenge slug"),
    ),
    responses(
        (status = OK, description = "Success", body = KubernetesActionResult, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Challenge not found", body = NotFoundError, content_type = "application/json"),
        (status = FORBIDDEN, description = "Forbidden", body = GenericError, content_type = "application/json")
    ),
    tag = "Challenge"
)]
async fn extend_challenge(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Extension(challenge): Extension<Challenge>,
) -> AxumResult<Json<KubernetesActionResult>> {
    state
        .orchestrator
        .add_time(challenge.id, *user_id, Duration::minutes(30))
        .await?;

    Ok(Json(KubernetesActionResult { success: true }))
}
