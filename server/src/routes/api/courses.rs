use axum::{Extension, Json};
use color_eyre::eyre::Context;
use futures::TryStreamExt;
use mongodb::bson::doc;
use utoipa_axum::routes;

use crate::{
    axum_error::AxumResult, database::Course, middlewares::require_auth::UnauthorizedError,
    routes::RouteProtectionLevel, state::AppState,
};

use super::Route;

const PATH: &str = "/api/courses";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_courses), RouteProtectionLevel::Authenticated)]
}

/// Get all courses
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = Vec<Course>, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Courses"
)]
async fn get_courses(Extension(state): Extension<AppState>) -> AxumResult<Json<Vec<Course>>> {
    let cursor = state
        .database
        .collection::<Course>("courses")
        .find(doc! {})
        .await
        .wrap_err("Failed to fetch courses")?;

    let courses = cursor.try_collect().await?;

    Ok(Json(courses))
}
