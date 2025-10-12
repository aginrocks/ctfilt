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
use tokio::select;
use tracing::info;
use utoipa_axum::{router::OpenApiRouter, routes};

use crate::{
    database::User,
    middlewares::require_auth::UserData,
    orchestrator::PodEvent,
    routes::api::ws::types::{ChallengesUpdate, ServerMessage, VpnState},
    state::AppState,
    vpn::models::{VpnEvent, VpnEventCore},
};

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().routes(routes!(websocket))
}

/// WebSocket
///
/// Sends real-time updates about challenges and other events.
#[utoipa::path(method(get), path = "/", tag = "WebSocket")]
async fn websocket(
    ws: WebSocketUpgrade,
    Extension(state): Extension<AppState>,
    Extension(user): Extension<UserData>,
) -> Response {
    info!("New WS Conenction");
    ws.on_upgrade(move |socket| handle_socket(socket, state, user.0))
}

async fn handle_socket(mut socket: WebSocket, state: AppState, user: User) {
    // Send latest challenges state
    let latest_challenges = state.orchestrator.watcher.get_latest_event(user.id);
    if let Ok(latest) = latest_challenges {
        handle_pod_event(latest, &mut socket).await.ok();
    }

    let latest_devices = state.vpn.get_devices(user.subject.clone()).await;
    if let Ok(devices) = latest_devices {
        handle_vpn_event(VpnEventCore::StateChanged { data: devices }, &mut socket)
            .await
            .ok();
    }

    let mut kube_rx = state.orchestrator.watcher.sender.subscribe();
    let mut vpn_rx = state.vpn.subscribe();
    loop {
        select! {
            Ok(event) = kube_rx.recv() => {
                if event.user != user.id {
                    continue;
                }
                handle_pod_event(event, &mut socket).await.ok();
            }
            Ok(event) = vpn_rx.recv() => {
                info!("Received VPN event: {:?}", event);
                if event.user_subject != user.subject {
                    continue;
                }
                handle_vpn_event(event.event, &mut socket).await.ok();
            }
        }
    }
}

async fn handle_pod_event(event: PodEvent, socket: &mut WebSocket) -> Result<()> {
    let msg = ServerMessage::ChallengesUpdate(ChallengesUpdate {
        challenges: event.challenges,
    });

    let msg = serde_json::to_string(&msg)?;
    socket.send(Message::Text(msg.into())).await?;

    Ok(())
}

async fn handle_vpn_event(event: VpnEventCore, socket: &mut WebSocket) -> Result<()> {
    let msg = match event {
        VpnEventCore::StateChanged { data } => ServerMessage::VpnState(VpnState { devices: data }),
        _ => todo!(),
    };

    let msg = serde_json::to_string(&msg)?;
    socket.send(Message::Text(msg.into())).await?;

    Ok(())
}
