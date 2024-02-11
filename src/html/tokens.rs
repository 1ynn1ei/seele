use std::mem;
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

    pub fn commit_buffer_to_attr_keys(&mut self) {
        self.tag.attr_keys.push(mem::take(&mut self.buffer));
    }

    pub fn commit_buffer_to_attr_value(&mut self) {
        self.tag.attr_values.push(mem::take(&mut self.buffer));
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

impl<'stream> Token<'stream> {

    fn printer_hepler(&self, data: &[&'stream u8]) -> String {
        String::from_utf8(data
            .iter()
            .map(|&x| *x)
            .collect()).unwrap()
    }

    pub fn present(&self) -> String {
        match self {
            Self::EndOfFile => String::from("EOF"),
            Self::Comment(data) => self.printer_hepler(data),
            Self::Character(byte) => {
                self.printer_hepler(&[*byte])
            },
            Self::EndTag(tag) |
            Self::StartTag(tag) => {
                let mut fmt_str = String::new();
                match self {
                    Self::EndTag(_) => fmt_str.push_str("EndTag: "),
                    Self::StartTag(_) => fmt_str.push_str("StartTag: "),
                    _ => {}
                }
                fmt_str.push_str(&self.printer_hepler(&tag.name));
                fmt_str.push('[');
                for i in 0..tag.attr_keys.len() {
                    fmt_str.push(' ');
                    let key = tag.attr_keys.get(i).unwrap();
                    let key_str = String::from_utf8(key
                                .iter()
                                .map(|&x| *x)
                                .collect()).unwrap();
                    fmt_str.push_str(&key_str);
                    fmt_str.push_str(": ");
                    match tag.attr_values.get(i) {
                        Some(bytes) => {
                            fmt_str.push_str(&String::from_utf8(bytes
                                        .iter()
                                        .map(|&x| *x)
                                        .collect()).unwrap());
                        },
                        None => {
                            fmt_str.push_str("EMPTY");
                        }
                    }
                }
                fmt_str.push(']');
                fmt_str
            }
            _ => String::from("TBD")
        }
    }
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

