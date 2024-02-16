use crate::html::dom::{
    DomObject,
    Node
};

pub struct HtmlElement {
    node: Node
}

impl HtmlElement {
    pub fn spawn() -> Self {
        Self {
            node: Node::default(),
        }
    }

    pub fn new() -> Box<Self> {
        Box::new(Self::spawn())
    }
}

impl DomObject for HtmlElement {}
