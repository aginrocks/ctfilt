use axum::{Extension, Json};
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    database::User,
    middlewares::require_auth::{UnauthorizedError, UserData},
    state::AppState,
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(get_user))
}

/// Get user details
#[utoipa::path(
    method(get),
    path = "/",
    responses(
        (status = OK, description = "Success", body = User, content_type = "application/json"),
        (status = UNAUTHORIZED, description = "Unauthorized", body = UnauthorizedError, content_type = "application/json")
    ),
    tag = "Auth"
)]
async fn get_user(Extension(user): Extension<UserData>) -> Json<User> {
    Json(user.0)
}
