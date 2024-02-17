use crate::html::dom::DomObject;

pub struct Comment {
    data: Option<String>,
}

impl Comment {
    pub fn spawn(
        data: Option<String>,
        ) -> Self {
        Self {
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
