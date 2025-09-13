mod courses;
mod health;
mod login;
mod schema;
mod user;

use super::Route;

pub fn routes() -> Vec<Route> {
    [
        health::routes(),
        user::routes(),
        login::routes(),
        schema::routes(),
        courses::routes(),
    ]
    .concat()
}
