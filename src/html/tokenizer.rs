use std::collections::HashMap;
//https://html.spec.whatwg.org/multipage/parsing.html#tokenization
pub struct DocType<'stream> {
    name: Option<&'stream [u8]>,
    public_id: Option<&'stream [u8]>,
    system_id: Option<&'stream [u8]>,
    force_quirks: bool,
}

impl<'stream> DocType<'stream> {
    pub fn new() -> Self {
        Self {
            name: None,
            public_id: None,
            system_id: None,
            force_quirks: false,
        }
    }
}

pub type AttrMap<'stream> = HashMap<&'stream [u8], &'stream [u8]>;
pub struct Tag<'stream> {
    name: &'stream [u8],
    self_closing: bool,
    attributes: AttrMap<'stream>
}

impl<'stream> Tag<'stream> {
    pub fn new(name: &'stream [u8]) -> Self {
        Self {
            name,
            self_closing: false, 
            attributes: AttrMap::new(),
        }
    }
}

pub enum Tokens<'stream> {
    Doctype(DocType<'stream>),
    StartTag(Tag<'stream>),
    EndTag(Tag<'stream>),
    Comment(&'stream [u8]),
    Character(&'stream [u8]),
    EndOfFile,
}
