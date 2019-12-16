#![feature(proc_macro_hygiene, decl_macro)]
use rocket::{routes};

mod memo;
mod utils;

use memo::controller;

fn main() {
    rocket::ignite()
        .mount("/", routes![
            controller::get_memo_list,
        ])
        .launch();
}
