use blitz::Viewport;
use blitz_dom::Document;

use crate::Config;
use crate::DocumentLike;

pub struct HtmlDocument {
    inner: Document,
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

impl HtmlDocument {
    pub(crate) fn from_html(html: &str, cfg: &Config) -> Self {
        // Spin up the virtualdom and include the default stylesheet
        let mut dom = Document::new(Viewport::new((0, 0)).make_device());

        // Set base url if configured
        if let Some(url) = &cfg.base_url {
            dom.set_base_url(&url);
        }

        // Include default and user-specified stylesheets
        dom.add_stylesheet(include_str!("./default.css"));
        for ss in &cfg.stylesheets {
            dom.add_stylesheet(&ss);
        }

        // Populate dom with HTML
        dom.write(&html);

        HtmlDocument { inner: dom }
    }
}
