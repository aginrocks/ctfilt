mod check;

use super::Route;

pub fn routes() -> Vec<Route> {
    [check::routes()].concat()
}
