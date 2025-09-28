use axum::{Extension, Json, extract::Path, middleware};
use color_eyre::eyre::{ContextCompat, eyre};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::{UnauthorizedError, UserId, require_auth},
    routes::api::{NotFoundError, challenges::challenge_slug::KubernetesActionResult},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(stop_challenge))
        .layer(middleware::from_fn(require_auth))
}

/// Stop a challenge
#[utoipa::path(
    method(post),
    path = "/",
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
async fn stop_challenge(
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Path(challenge_slug): Path<String>,
) -> AxumResult<Json<KubernetesActionResult>> {
    let challenge = state.store.challenges.get_by_slug(&challenge_slug).await?;

    let pod_name = state
        .orchestrator
        .watcher
        .users_state
        .get(&user_id)
        .ok_or_else(|| AxumError::not_found(eyre!("Challenge is not running")))?
        .value()
        .challenges
        .get(&challenge.id)
        .ok_or_else(|| AxumError::not_found(eyre!("Challenge is not running")))?
        .hostname
        .clone()
        .wrap_err("Missing challenge hostname")?;

    state
        .orchestrator
        .exterminator
        .exterminate(pod_name)
        .await?;

    Ok(Json(KubernetesActionResult { success: true }))
}
