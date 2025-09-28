mod types;

use axum::{
    Extension,
    extract::{
        WebSocketUpgrade,
        ws::{Message, WebSocket},
    },
    response::Response,
};
use color_eyre::eyre::Result;
use mongodb::bson::oid::ObjectId;
use tracing::info;
use utoipa_axum::routes;

use crate::{
    middlewares::require_auth::UserId,
    orchestrator::PodEvent,
    routes::{
        RouteProtectionLevel,
        api::ws::types::{ChallengesUpdate, ServerMessage},
    },
    state::AppState,
};

use super::Route;

const PATH: &str = "/api/ws";

pub fn routes() -> Vec<Route> {
    vec![(routes!(websocket), RouteProtectionLevel::Authenticated)]
}

/// Log in
#[utoipa::path(
    method(get),
    path = PATH,
    tag = "WebSocket"
)]
async fn websocket(
    ws: WebSocketUpgrade,
    Extension(state): Extension<AppState>,
    Extension(user_id): Extension<UserId>,
) -> Response {
    info!("New WS Conenction");
    ws.on_upgrade(move |socket| handle_socket(socket, state, *user_id))
}

async fn handle_socket(mut socket: WebSocket, state: AppState, user_id: ObjectId) {
    let latest = state.orchestrator.watcher.get_latest_event(user_id);
    if let Ok(latest) = latest {
        handle_event(latest, &mut socket).await.ok();
    }

    let mut rx = state.orchestrator.watcher.sender.subscribe();
    while let Ok(event) = rx.recv().await {
        if event.user != user_id {
            continue;
        }
        handle_event(event, &mut socket).await.ok();
    }
}

async fn handle_event(event: PodEvent, socket: &mut WebSocket) -> Result<()> {
    let msg = ServerMessage::ChallengesUpdate(ChallengesUpdate {
        challenges: event.challenges,
    });

    let msg = serde_json::to_string(&msg)?;
    socket.send(Message::Text(msg.into())).await?;

    Ok(())
}
