use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

mod lesson_slug;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new().nest("/{lesson_slug}", lesson_slug::routes())
}
