use crate::html::dom::DomObject;

pub struct HeadElement {
}

impl HeadElement {
    pub fn spawn() -> Self {
        Self {
        }
    }

    pub fn new() -> Box<Self> {
        Box::new(Self::spawn())
    }
}

impl DomObject for HeadElement {}
