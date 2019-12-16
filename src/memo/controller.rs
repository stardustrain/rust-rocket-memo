use rocket::{Request, Data};
use rocket::handler::Outcome;
use rocket::http::Status;
use log::{info};
use rocket_contrib::{json};
use serde_json;

use super::model::Memo;
use crate::utils::response;

pub fn get_memo_list<'s>(req: &'s Request, _: Data) -> Outcome<'s> {
    let memo = Memo::new("test", "test");
    let memo = vec![memo];
    let serialized = serde_json::to_string(&memo).unwrap();

    Outcome::from(req, json!({
        "memo": serialized,
    }))
}

pub fn get_memo<'s>(req: &'s Request, _: Data) -> Outcome<'s> {
    let memo_id = req.get_param::<u32>(1)
        .and_then(|r| r.ok());

    Outcome::Failure(Status::NotFound)
}
