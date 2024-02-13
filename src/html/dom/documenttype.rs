use crate::html::dom::{
    DomObject,
    Node
};

pub struct DocumentType {
    node: Node,
    name: String,
    public_id: String,
    system_id: String,
}

impl DocumentType {
    pub fn spawn(
        name: String,
        public_id: Option<String>,
        system_id: Option<String>
        ) -> Self {
        Self {
            node: Node::default(),
            name,
            public_id: public_id.unwrap_or_default(),
            system_id: system_id.unwrap_or_default(),
        }
    }

    pub fn new(
        name: String,
        public_id: Option<String>,
        system_id: Option<String>
        ) -> Box<Self> {
        Box::new(DocumentType::spawn(name, public_id, system_id))
    }
}

impl DomObject for DocumentType {}
