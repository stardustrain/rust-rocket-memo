use std::io::Cursor;
use rocket::response::Response;
use rocket::http::{Status, ContentType};
use rocket_contrib::{json};

pub fn response_builder(status: Status, message: &str) -> Response {
    let se = json!({
      "result": message
    });

    Response::build()
        .status(status)
        .header(ContentType::JSON)
        // .sized_body(Cursor::new(se))
        .finalize()
}
