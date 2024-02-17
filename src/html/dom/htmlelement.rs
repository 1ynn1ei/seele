use crate::html::dom::DomObject;

pub struct HtmlElement {
}

impl HtmlElement {
    pub fn spawn() -> Self {
        Self {
        }
    }

    pub fn new() -> Box<Self> {
        Box::new(Self::spawn())
    }
}

impl DomObject for HtmlElement {}
