use super::DocumentLike;
use blitz_dom::Document;

pub struct DioxusDocument {
    inner: Document,
}

// Implement DocumentLike and required traits for DioxusDocument

impl AsRef<Document> for DioxusDocument {
    fn as_ref(&self) -> &Document {
        &self.inner
    }
}
impl AsMut<Document> for DioxusDocument {
    fn as_mut(&mut self) -> &mut Document {
        &mut self.inner
    }
}
impl Into<Document> for DioxusDocument {
    fn into(self) -> Document {
        self.inner
    }
}
impl DocumentLike for DioxusDocument {}
