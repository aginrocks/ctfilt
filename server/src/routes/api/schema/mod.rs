mod challenge;
mod contest;
mod contest_batch;
mod course;
mod lesson;

use super::Route;

pub fn routes() -> Vec<Route> {
    [
        challenge::routes(),
        course::routes(),
        lesson::routes(),
        contest::routes(),
        contest_batch::routes(),
    ]
    .concat()
}
