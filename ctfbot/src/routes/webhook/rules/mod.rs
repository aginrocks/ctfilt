mod set_avatar;
mod template_pr;

use color_eyre::Result;

use crate::{routes::webhook::types::GiteaWebhookEvent, state::AppState};

pub async fn evaluate(state: AppState, webhook: GiteaWebhookEvent) -> Result<()> {
    set_avatar::evaluate(state.clone(), &webhook).await?;
    template_pr::evaluate(state.clone(), &webhook).await?;
    Ok(())
}
