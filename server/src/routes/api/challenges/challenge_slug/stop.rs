use axum::{Extension, Json, extract::Path};
use color_eyre::eyre::{ContextCompat, eyre};
use utoipa_axum::routes;

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::{
        RouteProtectionLevel,
        api::{NotFoundError, challenges::challenge_slug::KubernetesActionResult},
    },
    state::AppState,
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
