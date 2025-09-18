use axum::Json;
use manifests::CourseMetadata;
use schemars::schema_for;
use serde::Serialize;
use serde_json::Value;
use utoipa_axum::routes;

use crate::routes::RouteProtectionLevel;

use super::Route;

const PATH: &str = "/api/schema/course";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_course_schema), RouteProtectionLevel::Public)]
}

/// Course schema
///
/// This endpoint returns JSON schema for the course manifest.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = String, content_type = "application/json")
    ),
    tag = "Schema"
)]
pub async fn get_course_schema() -> Json<Value> {
    let schema = schema_for!(CourseMetadata);
    Json(
        schema
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize course JSON schema"),
    )
}
