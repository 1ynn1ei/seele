use crate::html::dom::DomObject;

pub struct Document {
}
impl Document {
    pub fn spawn() -> Self {
        Self {
        }
    }

    pub fn new() -> Box<Self> {
        Box::new(Self::spawn())
    }
}

impl DomObject for Document {}
