use crate::html::tokens::{Token, Tag, RefBuffer};
use crate::stream::Stream;
const LOWERCASE_OFFSET : u8 = 0x0020;

pub type TokenList<'stream> = Vec<Token<'stream>>;
pub enum States {
    Data,
    RCData,
    RawText,
    ScriptData,
    PlainText,
    TagOpen,
    EndTagOpen,
    TagName,
    RCDataLessThanSign,
    RCDataEndTagOpen,
    RCDataEndTagName,
    RawTextLessThanSign,
    RawTextEndTagOpen,
    RawTextEndTagName,
    ScriptDataLessThanSign,
    ScriptDataEndTagOpen,
    ScriptDataEndTagName,
    ScriptDataEscapeStart,
    ScriptDataEscapeStartDash,
    ScriptDataEscaped,
    ScriptDataEscapedDash,
    ScriptDataEscapedDashDash,
    ScriptDataEscapedLessThanSign,
    ScriptDataEscapedEndTagOpen,
    ScriptDataEscapedEndTagName,
    ScriptDataDoubleEscapeStart,
    ScriptDataDoubleEscaped,
    ScriptDataDoubleEscapedDash,
    ScriptDataDoubleEscapedDashDash,
    ScriptDataDoubleEscapedLessThanSign,
    ScriptDataDoubleEscapeEnd,
    BeforeAttributeName,
    AttributeName,
    AfterAttributeName,
    AttributeValueDoubleQuoted,
    AttributeValueSingleQuoted,
    AttributeValueUnquoted,
    AfterAttributeValueQuoted,
    SelfClosingStartTag,
    BogusComment,
    MarkupDeclarationOpen,
    CommentStart,
    CommentStartDash,
    Comment,
    CommentLessThanSign,
    CommentLessThanSignBang,
    CommentLessThanSignBangDash,
    CommentLessThanSignBangDashDash,
    CommentEndDash,
    CommentEnd,
    CommentEndBang,
    DocType,
    BeforeDocType,
    DocTypeName,
    AfterDocTypeName,
    AfterDocTypeNamePublicKeyword,
    BeforeDocTypePublicIdentifier,
    DocTypePublicIdentifierDoubleQuoted,
    DocTypePublicIdentifierSingleQuoted,
    AfterDocTypePublicIdentifier,
    BetweenDocTypePublicSystemIdentifiers,
    AfterDocTypeSystemKeyword,
    BeforeDocTypeSystemIdentifier,
    DocTypeSystemIdentifierDoubleQuoted,
    DocTypeSystemIdentifierSingleQuoted,
    AfterDocTypeSystemIdentifier,
    BogusDocType,
    CDataSection,
    CDataSectionBracket,
    CDataSectionEnd,
    CharacterReference,
    NamedCharacterReference,
    AmbiguousAmpersand,
    NumericCharacterReference,
    HexadecimalCharacterReferenceStart,
    DecimalCharacterReferenceStart,
    HexadecimalCharacterReference,
    DecimalCharacterReference,
    NumericCharacterReferenceEnd,
}

#[derive(Debug)]
pub enum TokenizerError {
    BufferPreparationConflict,
    BufferUnexpectedEmptyClear,
    BufferEmptyCommit,
    NamingOfNonTagToken,
}


pub struct Tokenizer<'stream> {
    stream: Stream<'stream>,
    state: States,
    return_state: States,
    tokens: TokenList<'stream>,
    buffer: RefBuffer<'stream, u8>,
    buffered_token: Option<Token<'stream>>,
}

impl<'stream> Tokenizer<'stream> {
    pub fn new(data: &'stream [u8]) -> Self {
        Self {
            stream: Stream::new(data),
            state: States::Data,
            return_state: States::Data,
            tokens: TokenList::new(),
            buffer: RefBuffer::new(),
            buffered_token: None,
        }
    }

    fn prepare_buffer(&mut self, token: Token<'stream>) -> Result<(), TokenizerError> {
        match self.buffered_token {
            Some(_) => {
                Err(TokenizerError::BufferPreparationConflict)
            },
            None => {
                self.buffered_token = Some(token);
                self.buffer = RefBuffer::new();
                Ok(())
            }
        }
    }

    fn clear_buffer(&mut self) -> Result<(), TokenizerError> {
        match self.buffered_token {
            Some(_) => {
                self.buffered_token = None;
                self.buffer = RefBuffer::new();
                Ok(())
            },
            None => {
                Err(TokenizerError::BufferUnexpectedEmptyClear)
            }
        }
    }

    fn push_to_buffer(&mut self, char: &'stream u8) {
        self.buffer.push(char);
    }

    fn commit_buffer(&mut self) -> Result<RefBuffer<'stream, u8>, TokenizerError> {
        match &self.buffered_token {
            Some(_) => {
                // TODO: this whole pattern is an absolute mess. clean it
                let slice = self.buffer.clone();
                self.buffer = RefBuffer::new(); 
                Ok(slice)
            },
            None => {
                Err(TokenizerError::BufferEmptyCommit)
            }
        }
    }

    pub fn make_tokens(&mut self) -> Result<&TokenList<'stream>, TokenizerError> {
        // check EOF before rest
        loop {
            if self.stream.is_eof() {
                // if EOF, go into EOF handler. some states create errors
                self.tokens.push(Token::EndOfFile);
                return Ok(&self.tokens);
            } else {
                let char = self.stream.current();
                match &self.state {
                    States::Data => self.data_state(char)?,
                    States::RCData => todo!(),
                    States::RawText => todo!(),
                    States::ScriptData => todo!(),
                    States::PlainText => todo!(),
                    States::TagOpen => self.tag_open_state(char)?,
                    States::EndTagOpen => todo!(),
                    States::TagName => self.tag_name_state(char)?,
                    _ => todo!(),
                }
            }
        }
    }

    
    //https://html.spec.whatwg.org/multipage/parsing.html#data-state
    fn data_state(&mut self, char: &'stream u8) -> Result<(), TokenizerError> {
        self.stream.advance();
        match char {
            b'&' => {
                self.return_state = States::Data;
                self.state = States::CharacterReference
            },
            b'<' => {
                self.state = States::TagOpen;
            },
            b'\0' => {
                // TODO: log unexpected-null-character error
                self.tokens.push(Token::Character(char));
            },
            _ => {
                self.tokens.push(Token::Character(char));
            }
        }
        Ok(())
    }
    //https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
    fn tag_open_state(&mut self, char: &'stream u8) -> Result<(), TokenizerError> {
        self.stream.advance();
        match char {
            b'!' => {
                self.state = States::MarkupDeclarationOpen;
            },
            b'/' => {
                self.state = States::EndTagOpen;
            },
            b'?' => {
                self.state = States::BogusComment;
                // self.prepare_buffer(Token::Comment(RefBuffer::new()))?;
                self.stream.reconsume();
            },
            b'a'..=b'z' | b'A'..=b'Z' => {
                self.state = States::TagName;
                // self.prepare_buffer(Token::StartTag(Tag::new()))?;
                self.stream.reconsume();
            },
            _ => {
                // TODO: invalid-first-character-of-tag-name error
                self.state = States::Data;
                // self.tokens.push(Token::Character(&b'<'));
                self.stream.reconsume();
            }
        }
        Ok(())
    }

    fn end_tag_open_state(&mut self, char: &'stream u8) -> Result<(), TokenizerError> {
        self.stream.advance();
        match char {
            b'a'..=b'z' | b'A'..=b'Z' => {
                self.state = States::TagName;
                self.prepare_buffer(Token::EndTag(Tag::new()))?;
                self.stream.reconsume();
            },
            b'>' => {
                // TODO: missing-end-tag-name error
                self.state = States::Data;
            },
            _ => {
                // TODO: invalid-first-character-of-tag-name error
                self.state = States::BogusComment;
                self.prepare_buffer(Token::Comment(RefBuffer::new()))?;
                self.stream.reconsume();
            }
        }
        Ok(())
    }

    fn tag_name_state(&mut self, char: &'stream u8) -> Result<(), TokenizerError> {
        self.stream.advance();
        match char {
            b'\t' |
            0x0A /* LF */ |
            0x0C /* FF */ |
            b' ' => { self.state = States::BeforeAttributeName; },
            b'/' => { self.state = States::SelfClosingStartTag; },
            b'>' => {
                self.state = States::Data;
                // TODO: actually emit here
            },
            b'A'..=b'Z' => {
                // TODO: handle lowercasing of tags during the tree creation
                // self.push_to_buffer(char);
            },
            b'\0' => {
                // TODO: unexpected-null-character error
                // self.push_to_buffer(&0xFF);
                // self.push_to_buffer(&0xFD);
            },
            _ => {
                // self.push_to_buffer(char);
            },
        }
        Ok(())
    }
}
