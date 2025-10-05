use color_eyre::{Result, eyre::ContextCompat};
use gitea_client::{
    apis::{
        configuration::Configuration,
        repository_api::{list_forks, repo_create_pull_request},
    },
    models::{CreatePullRequestOption, Repository},
};
use tracing::info;

use crate::{
    routes::webhook::types::{GiteaWebhookEvent, PushEventPayload},
    state::AppState,
};

pub async fn evaluate(state: AppState, webhook: &GiteaWebhookEvent) -> Result<()> {
    if let GiteaWebhookEvent::Push(payload) = webhook
        && payload.repository.forks_count != 0
        && state
            .settings
            .repositories
            .templates
            .contains(&payload.repository.full_name)
    {
        let forks = list_forks(
            &state.gitea,
            &payload.repository.owner.username,
            &payload.repository.name,
            Some(1),
            Some(9999),
        )
        .await?;
        info!("Got forks");
        dbg!(&forks);

        for fork in forks {
            dbg!(&fork);
            // if let Some(owner) = &fork.owner
            //     && owner.login != Some(payload.repository.owner.username.clone())
            // {
            //     info!("Skipping fork");
            //     continue;
            // }

            info!("Opening PR");
            open_template_pr(&state, payload, &fork).await?;
        }
    }
    Ok(())
}

pub async fn open_template_pr(
    state: &AppState,
    payload: &PushEventPayload,
    repo: &Repository,
) -> Result<()> {
    let owner = repo
        .owner
        .clone()
        .wrap_err("No owner")?
        .login
        .wrap_err("No owner login")?;
    let repo_name = repo.name.clone().wrap_err("No repo name")?;

    let body = format!(
        "This is an automated pull request to update the challenge with the latest changes from the [template repository]({}).\n\n\
Please review the changes and merge if everything looks good.\n\n
**Triggered by a push from:**d @{}",
        payload.repository.html_url,
        payload.pusher.username
    );

    let body = CreatePullRequestOption {
        head: Some(format!(
            "{owner}/{repo_name}:{branch}",
            branch = payload.git_ref.replace("refs/heads/", "")
        )),
        body: Some(body),
        base: Some(payload.git_ref.clone().replace("refs/heads/", "")),
        title: Some("Update template".into()),
        assignees: Some(vec!["Tymeksdfgsdfg".to_string()]),
        ..Default::default()
    };
    dbg!(&body);
    if let Err(e) = repo_create_pull_request(&state.gitea, &owner, &repo_name, Some(body)).await {
        dbg!(&e);
    }

    Ok(())
}
