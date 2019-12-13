#![feature(proc_macro_hygiene, decl_macro)]
use rocket::Route;

mod memo;

fn get_routes() -> Vec<Route> {
    memo::get_memo_routes()
}

fn main() {
    rocket::ignite().mount("/", get_routes()).launch();
}
