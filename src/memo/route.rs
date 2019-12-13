use rocket::Route;
use rocket::http::Method::*;

use super::controller;

pub fn get_memo_routes() -> Vec<Route> {
    vec![
        Route::new(Get, "/world/<name>", controller::hi)
    ]
}
