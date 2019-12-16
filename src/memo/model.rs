use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Memo {
    title: String,
    content: String,
    created_at: DateTime<Utc>,
    modified_at: DateTime<Utc>,
}

impl Memo {
    pub fn new(title: &str, content: &str) -> Memo {
        Memo {
          title: title.to_string(),
          content: content.to_string(),
          created_at: Utc::now(),
          modified_at: Utc::now(),
        }
    }
}
