mod html_document;
mod dioxus_document;

use blitz_dom::Document;

pub (crate) trait DocumentLike : AsRef<Document> + AsMut<Document> + Into<Document> {}

impl DocumentLike for Document {}
