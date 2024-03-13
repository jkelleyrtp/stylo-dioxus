use blitz_dom::Document;
use super::DocumentLike;


pub struct HtmlDocument {
    inner: Document
}

// Implement DocumentLike and required traits for HtmlDocument

impl AsRef<Document> for HtmlDocument {
    fn as_ref(&self) -> &Document {
        &self.inner
    }
}
impl AsMut<Document> for HtmlDocument {
    fn as_mut(&mut self) -> &mut Document {
        &mut self.inner
    }
}
impl Into<Document> for HtmlDocument {
    fn into(self) -> Document {
        self.inner
    }
}
impl DocumentLike for HtmlDocument {}