use axum::{Extension, Json, extract::Path};
use axum_valid::Valid;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::AxumResult,
    database::PartialSubmission,
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::{RouteProtectionLevel, api::NotFoundError},
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

#[derive(Deserialize, ToSchema, Validate)]
pub struct FlagSubmissionRequest {
    #[validate(length(min = 1))]
    pub flag: String,
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
    Extension(user_id): Extension<UserId>,
    Valid(Json(body)): Valid<Json<FlagSubmissionRequest>>,
) -> AxumResult<Json<FlagSubmissionResult>> {
    let challenge = state.store.challenges.get_by_slug(&challenge_slug).await?;

    let flags = state
        .flags
        .generate_all(*user_id, challenge.id, challenge.metadata.flags);

    let correct_flag = flags.into_iter().find(|f| f.value == body.flag);

    let submission = PartialSubmission {
        user: *user_id,
        challenge: challenge.id,
        flag: correct_flag.as_ref().map(|flag| flag.meta.slug.clone()),
        submitted_at: Utc::now(),
        raw_submission: body.flag,
        correct: correct_flag.is_some(),
    };

    state.store.submissions.add_submission(submission).await?;

    Ok(Json(FlagSubmissionResult {
        correct: correct_flag.is_some(),
        points_awarded: 0,
    }))
}
