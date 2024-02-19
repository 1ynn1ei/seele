use crate::html::dom::DomObject;

pub struct Element {
    prefix: Option<String>,
    tag_name: String,
    id: String,
    class_list: Vec<String>,
}

impl Element {
    pub fn spawn(
        tag_name: String,
        id: String,
        classes: String,
        ) -> Self {
        Self {
            prefix: None,
            tag_name,
            id,
            class_list: classes
                .split_whitespace()
                .map(|s| s.to_string())
                .collect(),
        }
    }

    pub fn new(
        tag_name: String,
        id: String,
        classes: String,
        ) -> Box<Self> {
        Box::new(Self::spawn(tag_name, id, classes))
    }
}

impl DomObject for Element {}
