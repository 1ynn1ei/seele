mod documenttype;
mod htmlelement;
mod headelement;
mod comment;
mod document;

pub use documenttype::DocumentType;
pub use htmlelement::HtmlElement;
pub use headelement::HeadElement;
pub use document::Document;
pub use comment::Comment;

use crate::html::HTMLError;

use crate::arena::{ArenaRef, Arena};

pub struct DomTree {
    root: Option<ArenaRef>,
    doctype: Option<ArenaRef>,
    arena: Arena<DomNode>,
}

impl DomTree {
    pub fn new(root: Box<dyn DomObject>) -> Self {
        let mut arena = Arena::new();
        let root_ref = arena.add(DomNode::new(root));
        Self {
            root: Some(root_ref),
            doctype: None,
            arena,
        }
    }

    pub fn insert (
            &mut self, 
            obj: Box<dyn DomObject>, 
            parent_ref: ArenaRef) -> Result<ArenaRef, HTMLError> {
        let mut node = DomNode::new(obj);
        node.parent = Some(parent_ref);
        let child_ref : ArenaRef = self.arena.add(node);
        if let Some(parent) = self.arena.get_mut(parent_ref) {
            parent.children.push(child_ref);
            Ok(child_ref)
        } else {
            Err(HTMLError::InaccessibleDomTreeNode)
        }
    }

    pub fn set_doctype(&mut self, doctype: ArenaRef) {
        self.doctype = Some(doctype);
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

