use axum::Json;
use manifests::ChallengeMetadata;
use schemars::schema_for;
use serde::Serialize;
use serde_json::Value;
use utoipa_axum::routes;

use crate::routes::RouteProtectionLevel;

use super::Route;

const PATH: &str = "/api/schema/challenge";

pub fn routes() -> Vec<Route> {
    vec![(routes!(get_challenge_schema), RouteProtectionLevel::Public)]
}

/// Get challenge manifest schema
///
/// This endpoint returns JSON schema for the challenge manifest.
#[utoipa::path(
    method(get),
    path = PATH,
    responses(
        (status = OK, description = "Success", body = String, content_type = "application/json")
    ),
)]
pub async fn get_challenge_schema() -> Json<Value> {
    let schema = schema_for!(ChallengeMetadata);
    Json(
        schema
            .serialize(serde_json::value::Serializer)
            .expect("failed to serialize challenge JSON schema"),
    )
}
