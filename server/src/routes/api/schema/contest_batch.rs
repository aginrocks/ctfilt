use axum::Json;
use manifests::ContestBatch;
use schemars::schema_for;
use serde::Serialize;
use serde_json::Value;
use utoipa_axum::routes;

use crate::routes::RouteProtectionLevel;

use super::Route;

const PATH: &str = "/api/schema/contest-batch";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_contest_schema), RouteProtectionLevel::Public)]
}

/// Batch schema
///
/// This endpoint returns JSON schema for the contest batch manifest.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = String, content_type = "application/json")
    ),
    tag = "Schema"
)]
pub async fn get_contest_schema() -> Json<Value> {
    let schema = schema_for!(ContestBatch<String>);
    Json(
        schema
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize contest batch JSON schema"),
    )
}
