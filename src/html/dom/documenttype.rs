use crate::html::dom::DomObject;

pub struct DocumentType {
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
        Box::new(Self::spawn(name, public_id, system_id))
    }
}

impl DomObject for DocumentType {}
