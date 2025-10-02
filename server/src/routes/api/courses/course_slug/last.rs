use axum::{Extension, Json, extract::Path};
use axum_valid::Valid;
use manifests::CourseItemType;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult,
    database::{Course, LastItem},
    middlewares::require_auth::{UnauthorizedError, UserId},
    routes::api::{CreateSuccess, NotFoundError},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(save_last))
}

/// Save last viewed item
///
/// Saves the last viewed lesson or challenge in the specified course
#[utoipa::path(
    method(put),
    path = "/",
    params(
        ("course_slug" = String, Path, description = "Course slug"),
    ),
    responses(
        (status = OK, description = "Success", body = Course, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Course not found", body = NotFoundError, content_type = "application/json")
    ),
    request_body = LastItem,
    tag = "Course"
)]
async fn save_last(
    Path(course_slug): Path<String>,
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
    Valid(Json(body)): Valid<Json<LastItem>>,
) -> AxumResult<Json<CreateSuccess>> {
    let course = state.store.courses.get_by_slug(&course_slug).await?;

    // Ensure the lesson or challenge exists
    match body.r#type {
        CourseItemType::Lesson => {
            state
                .store
                .lessons
                .get_by_slug(&course_slug, &body.slug)
                .await?;
        }
        CourseItemType::Challenge => {
            state.store.challenges.get_by_slug(&body.slug).await?;
        }
    }

    state
        .store
        .course_participations
        .update(*user_id, course.id, body.clone())
        .await?;

    Ok(Json(CreateSuccess {
        success: true,
        id: body.slug,
    }))
}
