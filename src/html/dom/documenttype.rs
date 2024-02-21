#[derive(Debug)]
pub struct DocumentType {
    name: String,
    public_id: String,
    system_id: String,
}

impl DocumentType {
    pub fn new(
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
}
