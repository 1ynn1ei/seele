mod documenttype;
mod htmlelement;
mod headelement;
mod comment;

pub use documenttype::DocumentType;
pub use htmlelement::HtmlElement;
pub use headelement::HeadElement;
pub use comment::Comment;

use crate::html::HTMLError;

use crate::arena::{ArenaRef, Arena};

pub struct DomTree {
    root: Option<ArenaRef>,
    arena: Arena<DomNode>
}

impl DomTree {
    pub fn new() -> Self {
        Self {
            root: None,
            arena: Arena::new(),
        }
    }

    pub fn insert<T:DomObject> (
            &mut self, 
            obj: T, 
            parent: ArenaRef) -> Result<ArenaRef, HTMLError> {
        let mut node = DomNode::new(Box::new(obj));
        node.parent = Some(parent);
        let child_ref : ArenaRef = self.arena.add(node);
        if let Some(parent) = self.arena.get_mut(parent) {
            parent.children.push(child_ref);
            Ok(child_ref)
        } else {
            Err(HTMLError::InaccessibleDomTreeNode)
        }
    }
}

pub struct DomNode {
    parent: Option<ArenaRef>,
    children: Vec<ArenaRef>,
    dom_obj: Box<dyn DomObject>
}

impl DomNode {
    pub fn new(obj: Box<dyn DomObject>) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            dom_obj: obj
        }
    }
}

pub trait DomObject {

}

#[derive(Default)]
pub struct Document {
    title: String,
    dir: String,
    body: Option<Box<dyn DomObject>>,
    // head: Option<Element>,
}
