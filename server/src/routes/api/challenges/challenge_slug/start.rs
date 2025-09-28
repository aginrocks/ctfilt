use axum::{Extension, Json, extract::Path, middleware};
use color_eyre::eyre::eyre;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::{AxumError, AxumResult},
    middlewares::require_auth::{UnauthorizedError, UserData, require_auth},
    routes::api::{
        GenericError, NotFoundError, challenges::challenge_slug::KubernetesActionResult,
    },
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .routes(routes!(start_challenge))
        .layer(middleware::from_fn(require_auth))
}

// TODO: Add rate limiting
/// Start a challenge
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
async fn start_challenge(
    Extension(state): Extension<AppState>,
    Path(course_slug): Path<String>,
    Extension(user): Extension<UserData>,
) -> AxumResult<Json<KubernetesActionResult>> {
    let challenge = state.store.challenges.get_by_slug(&course_slug).await?;

    let running_challenges = state.orchestrator.watcher.users_state.get(&user.id);
    if let Some(running_challenges) = running_challenges {
        let challenges = running_challenges.value().challenges.clone();
        if challenges.contains_key(&challenge.id) {
            return Err(AxumError::forbidden(eyre!("Challenge is already running")));
        }
        if challenges.len() >= state.settings.user_limits.max_concurrent_challenges {
            return Err(AxumError::forbidden(eyre!(
                "You have reached the maximum number of concurrent challenges ({}). Stop some challenges before starting new ones.",
                state.settings.user_limits.max_concurrent_challenges
            )));
        }
    }

    state
        .orchestrator
        .start_challenge(challenge.id, &challenge.metadata, user.id, &user.subject)
        .await?;

    Ok(Json(KubernetesActionResult { success: true }))
}
