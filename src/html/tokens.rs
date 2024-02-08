use std::{mem, collections::HashMap};
use crate::html::HTMLError;

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
    pub buffer: Vec<&'stream u8>,
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
    Comment(Vec<&'stream u8>),
    Character(&'stream u8),
    EndOfFile,
}

//https://html.spec.whatwg.org/multipage/parsing.html#tokenization
#[derive(Debug, Default)]
pub struct DocType<'stream> {
    name: Vec<&'stream u8>, 
    public_id: Vec<&'stream u8>, 
    system_id: Vec<&'stream u8>,
    force_quirks: bool,
}

#[derive(Debug, Default)]
pub struct Tag<'stream> {
    pub name: Vec<&'stream u8>,
    pub self_closing: bool,
    pub attr_keys: Vec<Vec<&'stream u8>>,
    pub attr_values: Vec<Vec<&'stream u8>>,
}

