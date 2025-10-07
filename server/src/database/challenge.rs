use color_eyre::eyre::{Context, eyre};
use futures::TryStreamExt;
use manifests::ChallengeMetadata;
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::{
    axum_error::{AxumError, AxumResult},
    mongo_id::object_id_as_string_required,
};

database_object!(Challenge {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(flatten)]
    metadata: ChallengeMetadata,

    r#ref: String,
});

#[derive(Clone)]
pub struct ChallengeStore {
    collection: Collection<Challenge>,
    partial_collection: Collection<PartialChallenge>,
}

impl ChallengeStore {
    pub fn new(database: &Database) -> Self {
        const COLLECTION: &str = "challenges";

        let collection = database.collection::<Challenge>(COLLECTION);
        let partial_collection = database.collection::<PartialChallenge>(COLLECTION);
        Self {
            collection,
            partial_collection,
        }
    }

    pub async fn get_by_slug(&self, slug: &str) -> AxumResult<Challenge> {
        let challenge = self
            .collection
            .find_one(doc! { "slug": slug })
            .await
            .wrap_err("Failed to fetch challenge")?
            .ok_or_else(|| AxumError::not_found(eyre!("Challenge not found")))?;

        Ok(challenge)
    }

    pub async fn get_many(&self, ids: Vec<ObjectId>) -> AxumResult<Vec<Challenge>> {
        let cursor = self
            .collection
            .find(doc! { "_id": { "$in": ids } })
            .await
            .wrap_err("Failed to fetch challenges")?;

        let challenges = cursor.try_collect().await?;
        Ok(challenges)
    }
    // Get the query to join public data
    // fn get_public_query() -> Vec<Document> {}
}
