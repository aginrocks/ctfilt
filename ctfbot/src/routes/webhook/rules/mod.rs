mod set_avatar;

use color_eyre::Result;

use crate::{routes::webhook::types::GiteaWebhookEvent, state::AppState};

pub async fn evaluate(state: AppState, webhook: GiteaWebhookEvent) -> Result<()> {
    set_avatar::evaluate(state, webhook).await?;
    Ok(())
}
