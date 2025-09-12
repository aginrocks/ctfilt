mod challenge;
mod course;

use super::Route;

pub fn routes() -> Vec<Route> {
    [challenge::routes(), course::routes()].concat()
}
