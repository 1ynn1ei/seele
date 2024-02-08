use crate::html::{
    tokens::{TokenBuilder, Token, TokenVariant, Tag, RefBuffer},
    HTMLError
};
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

pub struct Tokenizer<'stream> {
    stream: Stream<'stream>,
    state: States,
    return_state: States,
    tokens: TokenList<'stream>,
    builder: TokenBuilder<'stream>,
}

impl<'stream> Tokenizer<'stream> {
    pub fn new(data: &'stream [u8]) -> Self {
        Self {
            stream: Stream::new(data),
            state: States::Data,
            return_state: States::Data,
            tokens: TokenList::new(),
            builder: TokenBuilder::default(),
        }
    }

    pub fn make_tokens(&mut self) -> Result<&TokenList<'stream>, HTMLError> {
        // check EOF before rest
        loop {
            if self.stream.is_eof() {
                // if EOF, go into EOF handler. some states create errors
                self.tokens.push(Token::EndOfFile);
                return Ok(&self.tokens);
            } else {
                self.run_state()?;
            }
        }
    }

    pub fn run_state(&mut self) -> Result<(), HTMLError> {
        let char = self.stream.current();
        self.stream.advance();
        match &self.state {
            //https://html.spec.whatwg.org/multipage/parsing.html#data-state
            States::Data => {
                match char {
                    b'&' => {
                        self.return_state = States::Data;
                        self.state = States::CharacterReference;
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
            },
            States::RCData => todo!(),
            States::RawText => todo!(),
            States::ScriptData => todo!(),
            States::PlainText => todo!(),
            //https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
            States::TagOpen => {
                match char {
                    b'!' => {
                        self.state = States::MarkupDeclarationOpen;
                    },
                    b'/' => {
                        self.state = States::EndTagOpen;
                    },
                    b'?' => {
                        self.state = States::BogusComment;
                        self.builder.set_variant(TokenVariant::Comment)?;
                        self.stream.reconsume();
                    },
                    b'a'..=b'z' | b'A'..=b'Z' => {
                        self.state = States::TagName;
                        self.builder.set_variant(TokenVariant::StartTag)?;
                        self.stream.reconsume();
                    },
                    _ => {
                        // TODO: invalid-first-character-of-tag-name error
                        self.state = States::Data;
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.stream.reconsume();
                    }
                }
            },
            States::EndTagOpen => {
                match char {
                    b'a'..=b'z' | b'A'..=b'Z' => {
                        self.state = States::TagName;
                        self.builder.set_variant(TokenVariant::EndTag)?;
                        self.stream.reconsume();
                    },
                    b'>' => {
                        // TODO: missing-end-tag-name error
                        self.state = States::Data;
                    },
                    _ => {
                        // TODO: invalid-first-character-of-tag-name error
                        self.state = States::BogusComment;
                        self.builder.set_variant(TokenVariant::Comment)?;
                        self.stream.reconsume();
                    }
                }
            },
            States::TagName => {
                match char {
                    b'\t' |
                    0x0A /* LF */ |
                    0x0C /* FF */ |
                    b' ' => { self.state = States::BeforeAttributeName; },
                    b'/' => { self.state = States::SelfClosingStartTag; },
                    b'>' => {
                        self.state = States::Data;
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    },
                    b'A'..=b'Z' => {
                        // TODO: handle lowercasing of tags during the tree creation
                        self.builder.tag.name.push(char);
                    },
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        self.builder.tag.name.push(&0xFF);
                        self.builder.tag.name.push(&0xFD);
                    },
                    _ => {
                        self.builder.tag.name.push(char);
                    },
                }
            },
            _ => todo!(),
        }
        Ok(())
    }
}
