mod lesson_slug;

use super::Route;

pub fn routes() -> Vec<Route> {
    [lesson_slug::routes()].concat()
}
