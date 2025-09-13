use axum::{Extension, Json, extract::Path};
use manifests::LessonMetadata;
use serde_json::Value;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult,
    middlewares::require_auth::UnauthorizedError,
    routes::{RouteProtectionLevel, api::NotFoundError},
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/courses/{course_slug}/lessons";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_course_lessons), RouteProtectionLevel::Public)]
}

/// Get lessons
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("course_slug" = String, Path, description = "Course slug"),
    ),
    responses(
        (status = OK, description = "Success", body = Vec<LessonMetadata>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Course not found", body = NotFoundError, content_type = "application/json")
    ),
    tag = "Course"
)]
pub async fn get_course_lessons(
    Extension(state): Extension<AppState>,
    Path(course_slug): Path<String>,
) -> AxumResult<Json<Value>> {
    let course = state.store.courses.get_by_slug(&course_slug).await?;

    // let cursor = state
    //     .database
    //     .collection::<Lesson>("lessons")
    //     .find(doc! { "course":  })
    //     .await
    //     .wrap_err("Failed to fetch lessons")?;
    todo!()
}
