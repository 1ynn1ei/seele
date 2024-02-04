use std::collections::HashMap;
pub type AttrMap<'a> = HashMap<&'a [u8], &'a [u8]>;

pub struct Node<'a> {
    children: Vec<Node<'a>>,
    node_type: NodeType<'a>
}

pub enum NodeType<'a> {
    Text(&'a [u8]),
    Element(ElementData<'a>)
}

pub struct ElementData<'a> {
    tag_name: &'a [u8],
    attributes: AttrMap<'a>,
}

impl<'a> Node<'a> {
    fn text(data: &'a [u8]) -> Self {
        Node {
            children: Vec::new(),
            node_type: NodeType::Text(data)
        }
    }

    pub fn elem(name: &'a [u8], attrs: AttrMap<'a>, children: Vec<Node<'a>>) -> Self {
        Node {
            children,
            node_type: NodeType::Element(ElementData {
                tag_name: name,
                attributes: attrs,
            })
        }
    }

    pub fn get_tag_name(&self) -> &'a [u8] {
        match &self.node_type {
            NodeType::Text(_) => "text".as_bytes(),
            NodeType::Element(data) => {
                data.tag_name
            }
        }
    }
}

