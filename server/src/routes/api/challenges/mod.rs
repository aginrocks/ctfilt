mod challenge_slug;

use super::Route;

pub fn routes() -> Vec<Route> {
    [challenge_slug::routes()].concat()
}
