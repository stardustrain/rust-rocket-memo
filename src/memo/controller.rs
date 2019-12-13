use rocket::{Request, Data};
use rocket::handler::Outcome;
use rocket::http::RawStr;
use log::{info};

pub fn hi<'s>(req: &'s Request, _: Data) -> Outcome<'s> {
    let param = req.get_param::<&RawStr>(1)
        .and_then(|r| r.ok())
        .unwrap_or("unnamed".into());

    info!("{}", param);

    Outcome::from(req, "Hello!")
}
