use axum::{
    Extension,
    extract::{Path, Request},
    middleware::Next,
    response::Response,
};
use color_eyre::eyre::eyre;
use serde::Deserialize;

use crate::{
    axum_error::{AxumError, AxumResult},
    state::AppState,
};

pub mod challenge;
pub mod require_auth;

#[derive(Deserialize)]
pub struct Params {
    pub challenge_slug: String,
}

pub async fn challenge_middleware(
    Extension(state): Extension<AppState>,
    Path(Params { challenge_slug }): Path<Params>,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    let challenge = state.store.challenges.get_by_slug(&challenge_slug).await?;

    let courses = state
        .store
        .courses
        .count_with_challenge(challenge.id)
        .await?;
    if courses == 0 {
        return Err(AxumError::forbidden(eyre!(
            "You don't have access to this challenge (not part of any course or contest you're in)"
        )));
    }

    // TODO: Check contests

    request.extensions_mut().insert(challenge);

    Ok(next.run(request).await)
}
