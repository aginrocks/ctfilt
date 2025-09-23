use chrono::{DateTime, Utc};
use color_eyre::eyre::{Context, Result};
use mongodb::{
    Collection, Database,
    bson::{doc, oid::ObjectId},
};
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::mongo_id::object_id_as_string_required;

database_object!(Submission {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    user: ObjectId,

    #[serde(with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    challenge: ObjectId,

    /// Flag referenced by its slug
    flag: Option<String>,

    submitted_at: DateTime<Utc>,

    /// Raw text submitted by the user
    raw_submission: String,

    /// Whether the submission was correct
    correct: bool,
});

#[derive(Clone)]
pub struct SubmissionStore {
    collection: Collection<Submission>,
    partial_collection: Collection<PartialSubmission>,
}

impl SubmissionStore {
    pub fn new(database: &Database) -> Self {
        const COLLECTION: &str = "submissions";

        let collection = database.collection(COLLECTION);
        let partial_collection = database.collection(COLLECTION);
        Self {
            collection,
            partial_collection,
        }
    }

    pub async fn add_submission(&self, submission: PartialSubmission) -> Result<()> {
        self.partial_collection
            .insert_one(submission)
            .await
            .wrap_err("Failed to save submission")?;

        Ok(())
    }
}
