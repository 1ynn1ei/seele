use crate::html::dom::{
    DomObject,
    Node
};

pub struct Comment {
    node: Node,
    data: Option<String>,
}

impl Comment {
    pub fn spawn(
        data: Option<String>,
        ) -> Self {
        Self {
            node: Node::default(),
            data,
        }
    }

    pub fn new(
        data: Option<String>,
        ) -> Box<Self> {
        Box::new(Self::spawn(data))
    }
}
impl DomObject for Comment {}
