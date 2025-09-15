use std::ops::Deref;

use axum::{Extension, extract::Request, middleware::Next, response::Response};
use axum_oidc::OidcClaims;
use color_eyre::eyre::{self, ContextCompat, eyre};
use http::header::AUTHORIZATION;
use mongodb::{
    bson::{doc, oid::ObjectId},
    options::ReturnDocument,
};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::{
    GroupClaims,
    axum_error::{AxumError, AxumResult},
    database::{AccessToken, User},
    state::AppState,
    utils::hash_token,
};

/// User data type for request extensions
#[derive(Clone, Debug, Serialize, ToSchema, Deserialize)]
pub struct UserData(pub User);

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UserId(pub ObjectId);

impl Deref for UserData {
    type Target = User;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Deref for UserId {
    type Target = ObjectId;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Middleware that ensures the user is authenticated
pub async fn require_auth(
    claims: Option<OidcClaims<GroupClaims>>,
    Extension(state): Extension<AppState>,
    mut request: Request,
    next: Next,
) -> AxumResult<Response> {
    match claims {
        Some(claims) => {
            let sub = claims.subject().to_string();
            let name = claims
                .name()
                .wrap_err("Name is required")?
                .get(None)
                .wrap_err("Name is required")?
                .to_string();
            let email = claims.email().wrap_err("Email is required")?.to_string();

            let user = state
                .database
                .collection::<User>("users")
                .find_one_and_update(
                    doc! { "sub": &sub },
                    doc! {
                        "$set": {
                            "subject": sub,
                            "name": name,
                            "email": email,
                        }
                    },
                )
                .upsert(true)
                .return_document(ReturnDocument::After)
                .await?
                .wrap_err("User not found (wtf?")?;

            request.extensions_mut().insert(UserData(user.clone()));
            request.extensions_mut().insert(UserId(user.id));

            Ok(next.run(request).await)
        }
        None => require_system_auth(Extension(state), request, next).await,
    }
}

pub async fn require_system_auth(
    Extension(state): Extension<AppState>,
    request: Request,
    next: Next,
) -> AxumResult<Response> {
    let headers = request.headers();
    let auth_header = headers
        .get(AUTHORIZATION)
        .ok_or_else(|| AxumError::unauthorized(eyre!("Unauthorized")))?;

    let token = auth_header
        .to_str()
        .map_err(|_| AxumError::bad_request(eyre::eyre!("Unauthorized")))?
        .strip_prefix("Bearer ")
        .ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

    let hashed_token = hash_token(token);

    state
        .database
        .collection::<AccessToken>("tokens")
        .find_one(doc! { "hashed_token": hashed_token })
        .await?
        .ok_or_else(|| AxumError::unauthorized(eyre::eyre!("Unauthorized")))?;

    Ok(next.run(request).await)
}

#[derive(Serialize, ToSchema)]
#[schema(example = json!({"error": "Unauthorized"}))]
pub struct UnauthorizedError {
    error: String,
}
