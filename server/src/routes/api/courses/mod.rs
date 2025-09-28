mod course_slug;

use axum::{Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    axum_error::AxumResult, database::Course, middlewares::require_auth::UnauthorizedError,
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/{course_slug}", course_slug::routes())
        .routes(routes!(get_courses))
}

/// Get all courses
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = Vec<Course>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Courses"
)]
async fn get_courses(Extension(state): Extension<AppState>) -> AxumResult<Json<Vec<Course>>> {
    let courses = state.store.courses.get_all().await?;

    Ok(Json(courses))
}
