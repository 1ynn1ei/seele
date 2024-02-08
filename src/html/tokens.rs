use std::{mem, collections::HashMap};

pub type RefBuffer<'stream, T> = Vec<&'stream T>;

#[derive(Debug)]
pub enum TokenVariant {
    Doctype,
    StartTag,
    EndTag,
    Comment,
    Character,
    EndOfFile
}

pub struct TokenBuilder<'stream> {
    variant: TokenVariant,
    pub doctype: DocType<'stream>,
    pub tag: Tag<'stream>,
    pub buffer: RefBuffer<'stream, u8>,
}

impl<'stream> TokenBuilder<'stream> {
    pub fn new(variant: TokenVariant) -> Self {
        Self {
            variant,
            doctype: DocType::default(),
            tag: Tag::default(),
            buffer: RefBuffer::default(),
        }
    }

    pub fn build(&mut self) -> Token<'stream> {
        match self.variant {
            TokenVariant::Doctype => {
                Token::Doctype(mem::take(&mut self.doctype))
            },
            TokenVariant::StartTag => {
                Token::StartTag(mem::take(&mut self.tag))
            },
            TokenVariant::EndTag => {
                Token::EndTag(mem::take(&mut self.tag))
            },
            TokenVariant::Comment => {
                Token::Comment(mem::take(&mut self.buffer))
            },
            TokenVariant::Character => {
                Token::Character(self.buffer.pop().unwrap())
            },
            TokenVariant::EndOfFile => {
                Token::EndOfFile
            }
        }
    }
}

#[derive(Debug)]
pub enum Token<'stream> {
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

impl<'stream> Default for DocType<'stream> {
    fn default() -> Self {
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

impl<'stream> Default for Tag<'stream> {
    fn default() -> Self {
        Self {
            name: RefBuffer::new(),
            self_closing: false, 
            attributes: AttrMap::new(),
        }
    }
}
