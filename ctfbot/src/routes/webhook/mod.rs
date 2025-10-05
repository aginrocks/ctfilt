mod rules;
mod types;

use axum::{Extension, Router, body::Bytes, response::IntoResponse, routing::post};
use color_eyre::eyre::eyre;
use hmac::{Hmac, Mac};
use http::HeaderMap;
use sha2::Sha256;
use tracing::info;

use crate::{
    axum_error::{AxumError, AxumResult},
    routes::webhook::types::GiteaWebhookEvent,
    state::AppState,
};

pub fn routes() -> Router<AppState> {
    Router::new().route("/", post(handle_webhook))
}

pub fn verify_signature(secret: &str, signature: &str, body: &[u8]) -> AxumResult<()> {
    type HmacSha256 = Hmac<Sha256>;

    let mut mac = HmacSha256::new_from_slice(secret.as_bytes()).unwrap();
    mac.update(body);
    let expected = mac.finalize().into_bytes();
    let expected_hex = hex::encode(expected);
    let expected_signature = format!("sha256={expected_hex}");

    if signature == expected_signature {
        Ok(())
    } else {
        Err(AxumError::unauthorized(eyre!("Invalid webhook signature")))
    }
}

pub async fn handle_webhook(
    Extension(state): Extension<AppState>,
    headers: HeaderMap,
    body: Bytes,
) -> AxumResult<impl IntoResponse> {
    let secret = &state.settings.git.webhook_secret;

    let signature = headers
        .get("X-Hub-Signature-256")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            AxumError::bad_request(eyre!("Missing or invalid X-Hub-Signature-256 header"))
        })?;

    verify_signature(secret.as_str(), signature, &body)?;

    info!("body: {}", String::from_utf8_lossy(&body));

    let event = serde_json::from_slice::<GiteaWebhookEvent>(&body)?;

    dbg!(&event);
    rules::evaluate(state.clone(), event).await?;

    Ok(())
}
