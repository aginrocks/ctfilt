use axum::{Extension, Json, extract::Path};
use manifests::LessonMetadata;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    database::Lesson,
    middlewares::require_auth::UnauthorizedError,
    routes::{RouteProtectionLevel, api::NotFoundError},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/courses/{course_slug}/lessons/{lesson_slug}";

pub fn routes() -> Vec<Route> {
    vec![(
        routes!(get_course_lesson),
        RouteProtectionLevel::Authenticated,
    )]
}

/// Get lesson
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("course_slug" = String, Path, description = "Course slug"),
        ("lesson_slug" = String, Path, description = "Lesson slug"),
    ),
    responses(
        (status = OK, description = "Success", body = Vec<LessonMetadata>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Course or lesson not found", body = NotFoundError, content_type = "application/json")
    ),
    tag = "Course"
)]
pub async fn get_course_lesson(
    Extension(state): Extension<AppState>,
    Path((course_slug, lesson_slug)): Path<(String, String)>,
) -> AxumResult<Json<Lesson>> {
    let course = state.store.courses.get_by_slug(&course_slug).await?;
    let lesson = state
        .store
        .lessons
        .get_by_slug(course.id, &lesson_slug)
        .await?;

    Ok(Json(lesson))
}
