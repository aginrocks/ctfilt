use axum::{response::IntoResponse, routing::any};
use axum_oidc::handle_oidc_redirect;
use http::StatusCode;
use tracing::error;
use utoipa_axum::router::OpenApiRouter;

use crate::{GroupClaims, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().route(
        "/",
        any(|session, oidc_client, query| async move {
            match handle_oidc_redirect::<GroupClaims>(session, oidc_client, query).await {
                Ok(response) => response.into_response(),
                Err(e) => {
                    error!(error = ?e, "OIDC redirect handler error: {e}");
                    (StatusCode::BAD_REQUEST, format!("OIDC error: {e}")).into_response()
                }
            }
        }),
    )
}
