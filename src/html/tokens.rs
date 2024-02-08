use std::{mem, collections::HashMap};
use crate::html::HTMLError;

pub type RefBuffer<'stream, T> = Vec<&'stream T>;

#[derive(Debug)]
pub enum TokenVariant {
    Doctype,
    StartTag,
    EndTag,
    Comment,
    Character,
    EndOfFile,
}

#[derive(Default)]
pub struct TokenBuilder<'stream> {
    variant: Option<TokenVariant>,
    pub doctype: DocType<'stream>,
    pub tag: Tag<'stream>,
    pub buffer: RefBuffer<'stream, u8>,
}

impl<'stream> TokenBuilder<'stream> {
    pub fn set_variant(&mut self, variant: TokenVariant) -> Result<(), HTMLError> {
        match self.variant {
            Some(_) => Err(HTMLError::TokenBuilderImproperlyCleared),
            None => {
                self.variant = Some(variant);
                Ok(())
            }
        }
    }

    pub fn build(&mut self) -> Token<'stream> {
        // TODO: we need to properly handle error here! lol 
        match self.variant.as_mut().unwrap() {
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
#[derive(Debug, Default)]
pub struct DocType<'stream> {
    name: RefBuffer<'stream, u8>,
    public_id: RefBuffer<'stream, u8>,
    system_id: RefBuffer<'stream, u8>,
    force_quirks: bool,
}

pub type AttrMap<'stream> = HashMap<RefBuffer<'stream, u8>, RefBuffer<'stream, u8>>;

#[derive(Debug, Default)]
pub struct Tag<'stream> {
    pub name: RefBuffer<'stream, u8>,
    pub self_closing: bool,
    pub attributes: AttrMap<'stream>
}

