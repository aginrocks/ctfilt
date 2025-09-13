use axum::{Extension, Json, extract::Path};
use axum_valid::Valid;
use color_eyre::eyre::{Context, eyre};
use futures::TryStreamExt;
use manifests::CourseMetadata;
use mongodb::bson::doc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::Course,
    middlewares::require_auth::UnauthorizedError,
    routes::{
        RouteProtectionLevel,
        api::{CreateSuccess, NotFoundError},
    },
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/courses/{course_slug}";

pub fn routes() -> Vec<Route> {
    vec![
        (routes!(get_course), RouteProtectionLevel::Authenticated),
        (
            routes!(update_course),
            RouteProtectionLevel::SystemAuthenticated,
        ),
    ]
}

#[derive(Serialize, ToSchema)]
pub struct DetailedCourse {
    pub course: Course,
}

/// Get course
#[utoipa::path(
    method(get),
    path = PATH,
    params(
        ("course_slug" = String, Path, description = "Course slug"),
    ),
    responses(
        (status = OK, description = "Success", body = DetailedCourse, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Course not found", body = NotFoundError, content_type = "application/json")
    ),
    tag = "Courses"
)]
async fn get_course(
    Extension(state): Extension<AppState>,
    Path(course_slug): Path<String>,
) -> AxumResult<Json<DetailedCourse>> {
    let course = state
        .database
        .collection::<Course>("courses")
        .find_one(doc! { "slug": course_slug })
        .await
        .wrap_err("Failed to fetch courses")?
        .ok_or_else(|| AxumError::not_found(eyre!("Course not found")))?;

    let detailed = DetailedCourse { course };

    Ok(Json(detailed))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateCourseRequest {
    pub metadata: CourseMetadata,

    #[validate(length(min = 40, max = 64))]
    pub r#ref: String,
}

/// Update course
///
/// If course is not found, it will be created
#[utoipa::path(
    method(put),
    path = PATH,
    params(
        ("course_slug" = String, Path, description = "Course slug"),
    ),
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Courses"
)]
async fn update_course(
    Extension(state): Extension<AppState>,
    Path(course_slug): Path<String>,
    Valid(Json(body)): Valid<Json<UpdateCourseRequest>>,
) -> AxumResult<Json<CreateSuccess>> {
    todo!()
    // Ok(Json(CreateSuccess {
    //     success: true,
    // }))
}
