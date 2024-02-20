use crate::html::dom::DomObject;

pub struct Text {
    data: String
}

impl Text {
    pub fn spawn(data: String) -> Self {
        Self {
            data
        }
    }

    pub fn new(data: String) -> Box<Self> {
        Box::new(Self::spawn(data))
    }
}

impl Text {
    pub fn push_to_data(&mut self, data: String) {
        self.data.push_str(&data)
    }
}

impl DomObject for Text {}
