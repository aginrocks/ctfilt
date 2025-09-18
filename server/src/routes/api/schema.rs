mod challenge;
mod course;
mod lesson;

use super::Route;

pub fn routes() -> Vec<Route> {
    [challenge::routes(), course::routes(), lesson::routes()].concat()
}
