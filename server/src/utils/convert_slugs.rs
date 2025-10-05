use async_trait::async_trait;
use manifests::{CourseItem, CourseMetadata};
use mongodb::bson::oid::ObjectId;

use crate::{axum_error::AxumResult, database::DatabaseStore};

#[async_trait]
pub trait ConvertSlugs<To> {
    async fn convert_slugs(&self, store: DatabaseStore) -> AxumResult<To>;
}

#[async_trait]
impl ConvertSlugs<CourseMetadata<CourseItem<ObjectId>>> for CourseMetadata<CourseItem<String>> {
    async fn convert_slugs(
        &self,
        store: DatabaseStore,
    ) -> AxumResult<CourseMetadata<CourseItem<ObjectId>>> {
        let mut result = Vec::new();

        let item = self.clone();

        // TODO: Batch the queries
        for item in item.items.clone() {
            match item {
                CourseItem::Lesson { lesson } => {
                    let lesson = store.lessons.get_by_slug(&self.slug, &lesson).await?;
                    result.push(CourseItem::<ObjectId>::Lesson { lesson: lesson.id })
                }
                // TODO
                CourseItem::Challenge { challenge } => {
                    let challenge = store.challenges.get_by_slug(&challenge).await?;
                    result.push(CourseItem::<ObjectId>::Challenge {
                        challenge: challenge.id,
                    })
                }
            }
        }

        Ok(CourseMetadata {
            name: item.name,
            slug: item.slug,
            tags: item.tags,
            description: item.description,
            objectives: item.objectives,
            prerequisites: item.prerequisites,
            difficulty: item.difficulty,
            items: result,
        })
    }
}

// TODO: Implement for `ContestBatch`
