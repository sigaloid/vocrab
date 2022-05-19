use askama::Template;
use thesaurus_web_lib::{Data, SynonymInfo};

#[derive(Template)]
#[template(path = "index.html")]
pub struct HelloTemplate;

#[derive(Template)]
#[template(path = "word.html")]
pub struct WordTemplate {
    data: Data,
}
// Allow word template to be created from a synonymInfo struct. This works because the WordTemplate template calls on the fields of data directly
impl From<SynonymInfo> for WordTemplate {
    fn from(x: SynonymInfo) -> Self {
        Self { data: x.data }
    }
}

#[derive(Template)]
#[template(path = "error.html")]
pub struct ErrorTemplate;
