mod auth;
mod challenges;
mod courses;
mod health;
mod login;
mod schema;
mod user;
mod ws;

use axum::middleware;
use serde::Serialize;
use utoipa::ToSchema;
use utoipa_axum::router::OpenApiRouter;

use crate::{middlewares::require_auth::require_auth, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    let auth = OpenApiRouter::new()
        .nest("/auth", auth::routes())
        .nest("/challenges", challenges::routes())
        .nest("/user", user::routes())
        .nest("/ws", ws::routes())
        .nest("/courses", courses::routes())
        .layer(middleware::from_fn(require_auth));

    let public = OpenApiRouter::new()
        .nest("/health", health::routes())
        .nest("/login", login::routes())
        .nest("/schema", schema::routes());

    auth.merge(public)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Not Found"}))]
pub struct NotFoundError {
    error: String,
}

#[derive(Serialize, ToSchema)]
pub struct GenericError {
    error: String,
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"success": true,"id": "60c72b2f9b1d8c001c8e4f5a"}))]
pub struct CreateSuccess {
    success: bool,
    id: String,
}
