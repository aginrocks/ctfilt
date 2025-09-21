use axum::{Extension, Json, extract::Path};
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::{UnauthorizedError, UserData},
    routes::{
        RouteProtectionLevel,
        api::{NotFoundError, challenges::challenge_slug::KubernetesActionResult},
    },
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/challenges/{challenge_slug}/start";

pub fn routes() -> Vec<Route> {
    [vec![(
        routes!(start_challenge),
        RouteProtectionLevel::Authenticated,
    )]]
    .concat()
}

/// Start a challenge
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
async fn start_challenge(
    Extension(state): Extension<AppState>,
    Path(course_slug): Path<String>,
    Extension(user): Extension<UserData>,
) -> AxumResult<Json<KubernetesActionResult>> {
    let challenge = state.store.challenges.get_by_slug(&course_slug).await?;

    state
        .orchestrator
        .start_challenge(challenge.id, &challenge.metadata, user.id, &user.subject)
        .await?;

    Ok(Json(KubernetesActionResult { success: true }))
}
