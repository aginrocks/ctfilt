use utoipa_axum::{router::OpenApiRouter, routes};

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_health))
}

/// Check server health
///
/// This endpoint returns `ok`
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = str)
    ),
    tag = "Other"
)]
async fn get_health() -> &'static str {
    "ok"
}
