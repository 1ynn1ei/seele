use std::collections::HashMap;

#[derive(Debug)]
pub enum Tokens<'stream> {
    Doctype(DocType<'stream>),
    StartTag(Tag<'stream>),
    EndTag(Tag<'stream>),
    Comment(&'stream [u8]),
    Character(&'stream u8),
    EndOfFile,
}



//https://html.spec.whatwg.org/multipage/parsing.html#tokenization
#[derive(Debug)]
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

#[derive(Debug)]
pub struct Tag<'stream> {
    name: Option<&'stream [u8]>,
    self_closing: bool,
    attributes: AttrMap<'stream>
}

impl<'stream> Tag<'stream> {
    pub fn new() -> Self {
        Self {
            name: None,
            self_closing: false, 
            attributes: AttrMap::new(),
        }
    }

    pub fn set_name(&mut self, data: &'stream [u8]) {
        self.name = Some(data);
    }
}


