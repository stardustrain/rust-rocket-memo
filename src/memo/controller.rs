use rocket::{get};
use rocket_contrib::json::Json;

use super::model::Memo;

// pub fn get_memo_list<'s>(req: &'s Request, _: Data) -> Outcome<'s> {
//     let memo = Memo::new("test", "test");
//     let memo = vec![memo];
//     let serialized = serde_json::to_string(&memo).unwrap();

//     Outcome::from(req, json!({
//         "memo": serialized,
//     }))
// }

// pub fn get_memo<'s>(req: &'s Request, _: Data) -> Outcome<'s> {
//     let memo_id = req.get_param::<u32>(1)
//         .and_then(|r| r.ok());

//     Outcome::from(req, response::response_builder(Status::NotFound, "not found"))
// }
#[get("/memo")]
pub fn get_memo_list() -> Option<Json<Vec<Memo>>> {
    let memo = Memo::new("test", "test");
    let memo = vec![memo];

    Some(Json(memo))
}
