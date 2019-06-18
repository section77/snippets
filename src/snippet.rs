//! models snippets
use chrono::serde::ts_seconds;
use chrono::{DateTime, Utc};
use rocket::http::RawStr;
use rocket::request::FromFormValue;
use serde::{Deserialize, Serialize};

pub type SnippetId = u64;

#[derive(Debug, Serialize, Deserialize)]
pub struct Snippet {
    pub id: SnippetId,
    #[serde(with = "ts_seconds")]
    pub ts: DateTime<Utc>,
    pub tags: Tags,
    pub content: String,
}

impl Snippet {
    pub fn new(id: SnippetId, tags: Tags, content: &str) -> Self {
        let ts = Utc::now();
        let content = content.to_string();
        Snippet {
            id,
            ts,
            tags,
            content,
        }
    }
}

/// free form tags
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Tags(Vec<String>);

/// tags are separated with comma in the ui.
/// this ~FromFormValue~ implementation parses the tags, and convert it
/// in a ~Tags~ datastructure.
impl<'a> FromFormValue<'a> for Tags {
    type Error = ();
    fn from_form_value(form_value: &'a RawStr) -> Result<Tags, ()> {
        let s = form_value.url_decode_lossy();
        Ok(Tags(
            s.split(',').map(|s| s.trim()).map(String::from).collect(),
        ))
    }
}
