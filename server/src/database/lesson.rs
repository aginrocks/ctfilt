use manifests::{CourseMetadata, LessonMetadata};
use mongodb::bson::oid::ObjectId;
use partial_struct::Partial;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use visible::StructFields;

use crate::mongo_id::object_id_as_string_required;

database_object!(Lesson {
    #[serde(rename = "_id", with = "object_id_as_string_required")]
    #[schema(value_type = String)]
    id: ObjectId,

    name: String,

    slug: String,

    content: String,

    attachments: Vec<String>,

    r#ref: String,
});
