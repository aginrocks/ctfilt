use axum::{Extension, Json, extract::Path};
use serde::Serialize;
use utoipa::ToSchema;
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

const PATH: &str = "/api/challenges/{challenge_slug}/submit";

pub fn routes() -> Vec<Route> {
    [vec![(
        routes!(submit_flag),
        RouteProtectionLevel::Authenticated,
    )]]
    .concat()
}

#[derive(Serialize, ToSchema)]
pub struct FlagSubmissionResult {
    pub correct: bool,
    pub points_awarded: i32,
}

// TODO: Add rate limiting
/// Submit a flag
///
/// Submits a flag for the specified challenge. If all correct flags have been submitted, the challenge instance will be stopped.
#[utoipa::path(
    method(post),
    path = PATH,
    params(
        ("challenge_slug" = String, Path, description = "Challenge slug"),
    ),
    responses(
        (status = OK, description = "Success", body = FlagSubmissionResult, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Challenge not found", body = NotFoundError, content_type = "application/json")
    ),
    tag = "Challenge"
)]
async fn submit_flag(
    Extension(state): Extension<AppState>,
    Path(challenge_slug): Path<String>,
    Extension(user): Extension<UserData>,
) -> AxumResult<Json<FlagSubmissionResult>> {
    let challenge = state.store.challenges.get_by_slug(&challenge_slug).await?;

    Ok(Json(FlagSubmissionResult {
        correct: true,
        points_awarded: 0,
    }))
}
