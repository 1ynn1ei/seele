use std::collections::HashMap;
pub type RefBuffer<'stream, T> = Vec<&'stream T>;

#[derive(Debug)]
pub enum Tokens<'stream> {
    Doctype(DocType<'stream>),
    StartTag(Tag<'stream>),
    EndTag(Tag<'stream>),
    Comment(RefBuffer<'stream, u8>),
    Character(&'stream u8),
    EndOfFile,
}



//https://html.spec.whatwg.org/multipage/parsing.html#tokenization
#[derive(Debug)]
pub struct DocType<'stream> {
    name: RefBuffer<'stream, u8>,
    public_id: RefBuffer<'stream, u8>,
    system_id: RefBuffer<'stream, u8>,
    force_quirks: bool,
}

impl<'stream> DocType<'stream> {
    pub fn new() -> Self {
        Self {
            name: RefBuffer::new(),
            public_id: RefBuffer::new(),
            system_id: RefBuffer::new(),
            force_quirks: false,
        }
    }
}

pub type AttrMap<'stream> = HashMap<RefBuffer<'stream, u8>, RefBuffer<'stream, u8>>;

#[derive(Debug)]
pub struct Tag<'stream> {
    name: RefBuffer<'stream, u8>,
    self_closing: bool,
    attributes: AttrMap<'stream>
}

impl<'stream> Tag<'stream> {
    pub fn new() -> Self {
        Self {
            name: RefBuffer::new(),
            self_closing: false, 
            attributes: AttrMap::new(),
        }
    }

    pub fn set_name(&mut self, data: RefBuffer<'stream, u8>) {
        self.name = data;
    }
}


