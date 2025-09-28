mod challenge;
mod contest;
mod contest_batch;
mod course;
mod lesson;

use utoipa_axum::router::OpenApiRouter;

use crate::state::AppState;

pub fn routes() -> OpenApiRouter<AppState> {
    OpenApiRouter::new()
        .nest("/challenge", challenge::routes())
        .nest("/course", course::routes())
        .nest("/lesson", lesson::routes())
        .nest("/contest", contest::routes())
        .nest("/contest-batch", contest_batch::routes())
}
