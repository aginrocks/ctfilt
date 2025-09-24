use axum::Json;
use manifests::ContestMetadata;
use schemars::schema_for;
use serde::Serialize;
use serde_json::Value;
use utoipa_axum::routes;

use crate::routes::RouteProtectionLevel;

use super::Route;

const PATH: &str = "/api/schema/contest";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_contest_schema), RouteProtectionLevel::Public)]
}

/// Contest schema
///
/// This endpoint returns JSON schema for the contest manifest.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = String, content_type = "application/json")
    ),
    tag = "Schema"
)]
pub async fn get_contest_schema() -> Json<Value> {
    let schema = schema_for!(ContestMetadata);
    Json(
        schema
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize contest JSON schema"),
    )
}
