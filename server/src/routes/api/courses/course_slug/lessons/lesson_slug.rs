use axum::{Extension, Json, extract::Path};
use axum_valid::Valid;
use color_eyre::eyre::{Context, eyre};
use manifests::LessonMetadata;
use mongodb::bson::doc;
use serde::Deserialize;
use utoipa::ToSchema;
use utoipa_axum::routes;
use validator::Validate;

use crate::{
    axum_error::{AxumError, AxumResult},
    database::{Lesson, PartialLesson},
    middlewares::require_auth::UnauthorizedError,
    routes::{
        RouteProtectionLevel,
        api::{CreateSuccess, NotFoundError},
    },
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/courses/{course_slug}/lessons/{lesson_slug}";

pub fn routes() -> Vec<Route> {
    vec![
        (
            routes!(get_course_lesson),
            RouteProtectionLevel::Authenticated,
        ),
        (
            routes!(update_course_lesson),
            RouteProtectionLevel::SystemAuthenticated,
        ),
    ]
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
    let lesson = state
        .store
        .lessons
        .get_by_slug(&course_slug, &lesson_slug)
        .await?;

    Ok(Json(lesson))
}

#[derive(Deserialize, Validate, ToSchema)]
pub struct UpdateLessonRequest {
    pub metadata: LessonMetadata,
    pub content: String,
}

/// Update lesson
///
/// If lesson is not found, it will be created
#[utoipa::path(
    method(put),
    path = PATH,
    params(
        ("course_slug" = String, Path, description = "Course slug"),
        ("lesson_slug" = String, Path, description = "Lesson slug"),
    ),
    responses(
        (status = OK, description = "Success", body = CreateSuccess, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Course"
)]
async fn update_course_lesson(
    Extension(state): Extension<AppState>,
    Path((course_slug, lesson_slug)): Path<(String, String)>,
    Valid(Json(body)): Valid<Json<UpdateLessonRequest>>,
) -> AxumResult<Json<CreateSuccess>> {
    if body.metadata.slug != lesson_slug {
        return Err(AxumError::bad_request(eyre!("Slugs are immutable")));
    }

    let new_lesson = PartialLesson {
        metadata: body.metadata,
        content: body.content,
        attachments: vec![],
        course_slug: course_slug.clone(),
    };

    state
        .database
        .collection::<PartialLesson>("lessons")
        .find_one_and_replace(
            doc! {
                "course_slug": course_slug,
                "slug": lesson_slug.clone(),
            },
            new_lesson,
        )
        .upsert(true)
        .await
        .wrap_err("Failed to update lesson")?;

    Ok(Json(CreateSuccess {
        success: true,
        id: lesson_slug,
    }))
}
