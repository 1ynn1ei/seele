mod documenttype;

pub use documenttype::DocumentType;

use crate::html::HTMLError;
use crate::arena::{ArenaRef, Arena};

#[derive(Debug)]
pub enum DomObject {
    Document,
    DocumentType(DocumentType),
    Element(String),
    Head,
    Text(String)
}

pub struct DomTree {
    root: Option<ArenaRef>,
    doctype: Option<ArenaRef>,
    head: Option<ArenaRef>,
    pub arena: Arena<DomNode>,
}

impl DomTree {
    pub fn new(root: DomObject) -> Self {
        let mut arena = Arena::new();
        let root_ref = arena.add(DomNode::new(root));
        Self {
            root: Some(root_ref),
            doctype: None,
            head: None,
            arena,
        }
    }

    pub fn insert (
            &mut self, 
            obj: DomObject, 
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

    pub fn set_head(&mut self, head: ArenaRef) {
        self.head = Some(head);
    }

    pub fn get_last_child_of(&mut self, node_ref: ArenaRef) -> &mut DomObject {
        let node = self.arena.get(node_ref).unwrap();
        let child_ref = node.children.last().unwrap();
        let mut child_node = self.arena.get_mut(*child_ref).unwrap();
        &mut child_node.dom_obj
    }
}

pub struct DomNode {
    parent: Option<ArenaRef>,
    pub children: Vec<ArenaRef>,
    pub dom_obj: DomObject
}

impl DomNode {
    pub fn new(obj: DomObject) -> Self {
        Self {
            parent: None,
            children: Vec::new(),
            dom_obj: obj
        }
    }
}
