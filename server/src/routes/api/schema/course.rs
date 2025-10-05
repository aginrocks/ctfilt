use axum::Json;
use manifests::{CourseItem, CourseMetadata};
use schemars::schema_for;
use serde::Serialize;
use serde_json::Value;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_course_schema))
}

/// Course schema
///
/// This endpoint returns JSON schema for the course manifest.
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = String, content_type = "application/json")
    ),
    tag = "Schema"
)]
pub async fn get_course_schema() -> Json<Value> {
    let schema = schema_for!(CourseMetadata<CourseItem<String>>);
    Json(
        schema
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize course JSON schema"),
    )
}
