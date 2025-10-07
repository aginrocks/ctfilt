mod callback;

use axum::{
    error_handling::HandleErrorLayer,
    response::{IntoResponse, Redirect},
};
use axum_oidc::{OidcLoginLayer, error::MiddlewareError};
use tower::ServiceBuilder;
use tracing::error;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{GroupClaims, state::AppState};

pub fn routes() -> OpenApiRouter<AppState> {
    let oidc_login_service = ServiceBuilder::new()
        .layer(HandleErrorLayer::new(|e: MiddlewareError| async {
            error!(error = ?e, "An error occurred in OIDC login middleware");
            e.into_response()
        }))
        .layer(OidcLoginLayer::<GroupClaims>::new());

    OpenApiRouter::new()
        .routes(routes!(log_in))
        .layer(oidc_login_service)
        .nest("/callback", callback::routes())
}

/// Log in
#[utoipa::path(method(get), path = "/", tag = "Auth")]
async fn log_in() -> impl IntoResponse {
    Redirect::to("/lab")
}
