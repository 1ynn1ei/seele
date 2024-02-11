pub enum Node {
    Document,
    Doctype,
    Element,
    Text
}
pub struct Document {
    title: String,
    dir: String,
    body: Option<Element>,
    head: Option<Element>,
}

pub struct Element {
    namespace_uri: Option<String>,
    prefix: Option<String>,
    local_name: String,
    tag_name: String,
    id: String,
    class_name: String,
    slot: String,
}

pub struct Text {
    whole_text: String,
}
