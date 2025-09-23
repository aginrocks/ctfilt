#[macro_use]
pub mod macros;

pub mod access_token;
pub mod challenge;
pub mod course;
pub mod init;
pub mod lesson;
pub mod submission;
pub mod user;

pub use access_token::*;
pub use challenge::*;
pub use course::*;
pub use init::*;
pub use lesson::*;
use mongodb::Database;
pub use submission::*;
pub use user::*;

#[derive(Clone)]
pub struct DatabaseStore {
    pub database: Database,
    pub courses: CourseStore,
    pub lessons: LessonStore,
    pub challenges: ChallengeStore,
    pub submissions: SubmissionStore,
}

impl DatabaseStore {
    pub fn new(database: &Database) -> Self {
        Self {
            database: database.clone(),
            courses: CourseStore::new(database),
            lessons: LessonStore::new(database),
            challenges: ChallengeStore::new(database),
            submissions: SubmissionStore::new(database),
        }
    }
}
