mod start;
mod stop;
mod submit;

use axum::{Extension, Json, extract::Path, middleware};
use axum_valid::Valid;
use color_eyre::eyre::{Context, eyre};
use manifests::ChallengeMetadata;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Challenge, PartialChallenge},
    middlewares::require_auth::{UnauthorizedError, require_system_auth},
    routes::api::{CreateSuccess, NotFoundError},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    let system = OpenApiRouter::new()
        .routes(routes!(update_challenge))
        .layer(middleware::from_fn(require_system_auth));

    OpenApiRouter::new()
        .merge(system)
        .routes(routes!(get_challenge))
        .nest("/start", start::routes())
        .nest("/stop", stop::routes())
        .nest("/submit", submit::routes())
}

/// Get challenge
#[utoipa::path(
    method(get),
    path = "/",
    params(
        ("challenge_slug" = String, Path, description = "Challenge slug"),
    ),
    responses(
        (status = OK, description = "Success", body = Challenge, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Challenge not found", body = NotFoundError, content_type = "application/json")
    ),
    tag = "Challenge"
)]
async fn get_challenge(
    Extension(state): Extension<AppState>,
    Path(course_slug): Path<String>,
) -> AxumResult<Json<Challenge>> {
    // TODO: Omit flags from response
    let challenge = state.store.challenges.get_by_slug(&course_slug).await?;

    Ok(Json(challenge))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateChallengeRequest {
    pub metadata: ChallengeMetadata,

    #[validate(length(min = 40, max = 64))]
    pub r#ref: String,
}

/// Update challenge
///
/// If challenge is not found, it will be created
#[utoipa::path(
    method(put),
    path = "/",
    params(
        ("challenge_slug" = String, Path, description = "Challenge slug"),
    ),
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Challenge"
)]
async fn update_challenge(
    Extension(state): Extension<AppState>,
    Path(challenge_slug): Path<String>,
    Valid(Json(body)): Valid<Json<UpdateChallengeRequest>>,
) -> AxumResult<Json<CreateSuccess>> {
    if body.metadata.slug != challenge_slug {
        return Err(AxumError::bad_request(eyre!("Slugs are immutable")));
    }

    let new_challenge = PartialChallenge {
        metadata: body.metadata,
        r#ref: body.r#ref,
    };

    state
        .database
        .collection::<PartialChallenge>("challenges")
        .find_one_and_replace(doc! { "slug": challenge_slug.clone() }, new_challenge)
        .upsert(true)
        .await
        .wrap_err("Failed to update course")?;

    Ok(Json(CreateSuccess {
        success: true,
        id: challenge_slug,
    }))
}

/// Further information can be obtained from Socket.IO connection
#[derive(Serialize, ToSchema)]
pub struct KubernetesActionResult {
    pub success: bool,
}
