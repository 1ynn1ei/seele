mod documenttype;
mod htmlelement;
mod headelement;
mod comment;

pub use documenttype::DocumentType;
pub use htmlelement::HtmlElement;
pub use headelement::HeadElement;
pub use comment::Comment;

pub trait DomObject {

}

#[derive(Default)]
pub struct Document {
    title: String,
    dir: String,
    body: Option<Box<dyn DomObject>>,
    // head: Option<Element>,
}
