use std::sync::LazyLock;

use base64::{Engine, engine::general_purpose};
use color_eyre::Result;
use gitea_client::{apis::repository_api::repo_update_avatar, models::UpdateRepoAvatarOption};
use tracing::info;

use crate::{
    routes::webhook::types::{GiteaWebhookEvent, RepositoryAction},
    state::AppState,
};

static CHALLENGE_AVATAR: LazyLock<String> = LazyLock::new(|| {
    let bytes = include_bytes!("avatar.png");

    general_purpose::STANDARD.encode(bytes)
});

pub async fn evaluate(state: AppState, webhook: &GiteaWebhookEvent) -> Result<()> {
    if let GiteaWebhookEvent::Repository(payload) = webhook
        && payload.action == RepositoryAction::Created
        && payload.repository.name.starts_with("challenge-")
    {
        info!("Created a matching repo");

        repo_update_avatar(
            &state.gitea,
            &payload.organization.username,
            &payload.repository.name,
            Some(UpdateRepoAvatarOption {
                image: Some(CHALLENGE_AVATAR.clone()),
            }),
        )
        .await?;
    }

    Ok(())
}
