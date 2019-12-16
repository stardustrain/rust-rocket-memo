use rocket::Route;
use rocket::http::Method::*;

use super::controller;

pub fn get_memo_routes() -> Vec<Route> {
    vec![
        Route::new(Get, "/memo", controller::get_memo_list),
        Route::new(Get, "/memo/<memo_id>", controller::get_memo),
    ]
}
