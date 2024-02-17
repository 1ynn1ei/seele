use crate::html::dom::{
    DomObject,
    Node
};

pub struct HeadElement {
    node: Node
}

impl HeadElement {
    pub fn spawn() -> Self {
        Self {
            node: Node::default(),
        }
    }

    pub fn new() -> Box<Self> {
        Box::new(Self::spawn())
    }
}

impl DomObject for HeadElement {}
