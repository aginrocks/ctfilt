mod webhook;

use axum::Router;

use crate::state::AppState;

pub fn routes() -> Router<AppState> {
    Router::new().nest("/webhook", webhook::routes())
}
