mod lessons;

use axum::{Extension, Json, extract::Path, middleware};
use axum_valid::Valid;
use color_eyre::eyre::{Context, eyre};
use manifests::CourseMetadata;
use mongodb::bson::doc;
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::{router::OpenApiRouter, routes};
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Course, PartialCourse},
    middlewares::require_auth::{UnauthorizedError, require_system_auth},
    routes::api::{CreateSuccess, NotFoundError},
    state::AppState,
    utils::ConvertSlugs,
};

pub fn routes() -> OpenApiRouter<AppState> {
    let system = OpenApiRouter::new()
        .routes(routes!(update_course))
        .layer(middleware::from_fn(require_system_auth));

    OpenApiRouter::new()
        .merge(system)
        .routes(routes!(get_course))
        .nest("/lessons", lessons::routes())
}

/// Get course
#[utoipa::path(
    method(get),
    path = "/",
    params(
        ("course_slug" = String, Path, description = "Course slug"),
    ),
    responses(
        (status = OK, description = "Success", body = Course, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json"),
        (status = NOT_FOUND, description = "Course not found", body = NotFoundError, content_type = "application/json")
    ),
    tag = "Course"
)]
async fn get_course(
    Extension(state): Extension<AppState>,
    Path(course_slug): Path<String>,
) -> AxumResult<Json<Course>> {
    let course = state.store.courses.get_by_slug(&course_slug).await?;

    Ok(Json(course))
}

#[derive(Deserialize, ToSchema, Validate)]
pub struct UpdateCourseRequest {
    pub metadata: CourseMetadata<String>,

    #[validate(length(min = 40, max = 64))]
    pub r#ref: String,
}

/// Update course
///
/// If course is not found, it will be created
#[utoipa::path(
    method(put),
    path = "/",
    params(
        ("course_slug" = String, Path, description = "Course slug"),
    ),
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Course"
)]
async fn update_course(
    Extension(state): Extension<AppState>,
    Path(course_slug): Path<String>,
    Valid(Json(body)): Valid<Json<UpdateCourseRequest>>,
) -> AxumResult<Json<CreateSuccess>> {
    if body.metadata.slug != course_slug {
        return Err(AxumError::bad_request(eyre!("Slugs are immutable")));
    }

    let new_course = PartialCourse {
        metadata: body.metadata.convert_slugs(state.store).await?,
        r#ref: body.r#ref,
    };

    state
        .database
        .collection::<PartialCourse>("courses")
        .find_one_and_replace(doc! { "slug": course_slug.clone() }, new_course)
        .upsert(true)
        .await
        .wrap_err("Failed to update course")?;

    Ok(Json(CreateSuccess {
        success: true,
        id: course_slug,
    }))
}
