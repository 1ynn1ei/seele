use crate::html::{
    tokens::{TokenBuilder, Token, TokenVariant },
    HTMLError
};
use crate::stream::Stream;
const LOWERCASE_OFFSET : u8 = 0x0020;

pub type TokenList<'stream> = Vec<Token<'stream>>;
#[derive(Debug)]
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
    BeforeAttributeValue,
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
    pub fn new(data: &'stream Vec<u8>) -> Self {
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
            println!("{:?}", self.state);
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
        match self.state {
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
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.builder.buffer.push(char);
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    },
                    _ => {
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.builder.buffer.push(char);
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    }
                }
            },

            States::RCData => {
                match char {
                    b'&' => {
                        self.return_state = States::RCData;
                        self.state = States::CharacterReference;
                    },
                    b'<' => self.state = States::RCDataLessThanSign,
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        self.builder.tag.name.push(&0xFF);
                        self.builder.tag.name.push(&0xFD);
                    },
                    _ => {
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.builder.buffer.push(char);
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    }
                }
            },
            
            States::RawText => {
                match char {
                    b'<' => self.state = States::RawTextLessThanSign,
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        self.builder.tag.name.push(&0xFF);
                        self.builder.tag.name.push(&0xFD);
                    },
                    _ => {
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.builder.buffer.push(char);
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    }
                }
            },

            States::ScriptData => {
                match char {
                    b'<' => self.state = States::ScriptDataLessThanSign,
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        self.builder.tag.name.push(&0xFF);
                        self.builder.tag.name.push(&0xFD);
                    },
                    _ => {
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.builder.buffer.push(char);
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    }
                }
            },

            States::PlainText => {
                match char {
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        self.builder.tag.name.push(&0xFF);
                        self.builder.tag.name.push(&0xFD);
                    },
                    _ => {
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.builder.buffer.push(char);
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    }
                }
            },

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
                    b'\n'/* LF */ |
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

            States::BeforeAttributeName => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => { },
                    b'/' |
                    b'>' => {
                        self.state = States::AfterAttributeName;
                        self.stream.reconsume();
                    },
                    b'=' => {
                        // TODO: unexpected-equqls-sign-before-attribute-name error
                        self.builder.buffer.push(char);
                        self.state = States::AttributeName;
                    },
                    _ => {
                        self.state = States::AttributeName;
                        self.stream.reconsume();
                    }
                }
            },

            States::AttributeName => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => { },
                    b'/' |
                    b'>' => {
                        self.state = States::AfterAttributeName;
                        self.stream.reconsume();
                    },
                    b'=' => {
                        // TODO: put the value name in, reset buffer
                        self.state = States::BeforeAttributeValue;
                    },
                    b'A'..=b'Z' => {
                        self.builder.buffer.push(char);
                    },
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        self.builder.buffer.push(&0xFF);
                        self.builder.buffer.push(&0xFD);
                    },
                    b'\''|
                    b'"' |
                    b'<' => {
                        // TODO: unexpected-character-in-attribute-name errorname
                        self.builder.buffer.push(char);
                    },
                    _ => {
                        self.builder.buffer.push(char);
                    }
                }
            },

            States::BeforeAttributeName => {
            },
            States::AfterAttributeName => {
            },
            States::BeforeAttributeValue => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => {  },
                    b'"' => self.state = States::AttributeValueDoubleQuoted,
                    b'\'' => self.state = States::AttributeValueSingleQuoted,
                    b'>' => {
                        // TODO: missing-attribute-value error
                        self.state = States::Data;
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    },
                    _ => {
                        self.state = States::AttributeValueUnquoted;
                        self.stream.reconsume();
                    }
                }
            },
            States::AttributeValueDoubleQuoted => {
                match char {
                    b'"' => self.state = States::AfterAttributeValueQuoted,
                    b'&' => {
                        self.return_state = States::AttributeValueDoubleQuoted;
                        self.state = States::CharacterReference;
                    },
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        self.builder.buffer.push(&0xFF);
                        self.builder.buffer.push(&0xFD);
                    },
                    _ => self.builder.buffer.push(char)
                }
            }
            States::AfterAttributeValueQuoted => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => self.state = States::BeforeAttributeName,
                    b'/' => self.state = States::SelfClosingStartTag,
                    b'>' => {
                        self.state = States::Data;
                        // TODO: actually put the darn value in !
                        self.tokens.push(self.builder.build());
                        self.builder = TokenBuilder::default();
                    },
                    _ => {
                        // TODO: missing-whitespace-between-attributes error
                        self.state = States::BeforeAttributeName;
                        self.stream.reconsume();
                    }
                }
            }
            _ => todo!(),
        }
        Ok(())
    }
}
