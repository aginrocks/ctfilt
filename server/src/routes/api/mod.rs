mod auth;
mod challenges;
mod courses;
mod health;
mod login;
mod schema;
mod user;
mod ws;

use serde::Serialize;
use utoipa::ToSchema;

use super::Route;

pub fn routes() -> Vec<Route> {
    [
        health::routes(),
        user::routes(),
        login::routes(),
        schema::routes(),
        courses::routes(),
        auth::routes(),
        challenges::routes(),
        ws::routes(),
    ]
    .concat()
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
