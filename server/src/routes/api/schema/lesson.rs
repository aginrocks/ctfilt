use axum::Json;
use manifests::LessonMetadata;
use schemars::schema_for;
use serde::Serialize;
use serde_json::Value;
use utoipa_axum::routes;

use crate::routes::RouteProtectionLevel;

use super::Route;

const PATH: &str = "/api/schema/lesson";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_lesson_schema), RouteProtectionLevel::Public)]
}

/// Lesson schema
///
/// This endpoint returns JSON schema for the lesson manifest.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = String, content_type = "application/json")
    ),
    tag = "Schema"
)]
pub async fn get_lesson_schema() -> Json<Value> {
    let schema = schema_for!(LessonMetadata);
    Json(
        schema
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize lesson JSON schema"),
    )
}
