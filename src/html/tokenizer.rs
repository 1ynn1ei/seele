use crate::html::{
    tokens::{TokenBuilder, Token, TokenVariant },
    HTMLError
};
use crate::stream::Stream;

pub type TokenList<'stream> = Vec<Token<'stream>>;
#[derive(Debug)]
#[allow(dead_code)]
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
    BeforeDocTypeName,
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
    pub state: States,
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

    pub fn get_next_token(&mut self) -> Result<Option<Token>, HTMLError> {
        if !self.tokens.is_empty() {
            Ok(self.tokens.pop())
        } else if self.stream.is_eof() {
            // TODO: we need to handle EOF differnet for some states
            Ok(Some(Token::EndOfFile))
        } else {
            println!("[TOKENIZER STATE:{:?}]", self.state);
            self.run_state()
        }
    }

    pub fn run_state(&mut self) -> Result<Option<Token>, HTMLError> {
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
                        return Ok(Some(self.builder.build()));

                    },
                    _ => {
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.builder.buffer.push(char);
                        return Ok(Some(self.builder.build()));
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
                        return Ok(Some(self.builder.build()));
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
                        return Ok(Some(self.builder.build()));
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
                        return Ok(Some(self.builder.build()));
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
                        return Ok(Some(self.builder.build()));
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
                        return Ok(Some(self.builder.build()));
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

            States::RCDataLessThanSign => {
                match char {
                    b'/' => {
                        self.state = States::RCDataEndTagOpen;
                    },
                    _ => {
                        self.builder.set_variant(TokenVariant::Character)?;
                        self.builder.buffer.push(&b'<');
                        self.state = States::RCData;
                        self.stream.reconsume();
                        return Ok(Some(self.builder.build()));
                    }
                }
            },
            States::RCDataEndTagOpen => {
                match char {
                    b'a'..=b'z' | b'A'..=b'Z' => {
                        self.builder.set_variant(TokenVariant::EndTag);
                        self.state = States::RCDataEndTagName;
                        self.stream.reconsume();
                    },
                    _ => todo!()
                }
            },
            States::RCDataEndTagName => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => {
                        if self.builder.check_tag_validitiy() {
                            self.state = States::BeforeAttributeName;
                        } else {
                            todo!()
                        }
                    },
                    b'/' => todo!(),
                    b'>' => {
                        if self.builder.check_tag_validitiy() {
                            self.state = States::Data;
                            return Ok(Some(self.builder.build()));
                        } else {
                            todo!()
                        }
                    },
                    b'a'..=b'z' | b'A'..=b'Z' => {
                        self.builder.tag.name.push(char);
                    },
                    _ => todo!()
                }
            },
            // RawTextLessThanSign,
            // RawTextEndTagOpen,
            // RawTextEndTagName,
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
                        self.builder.commit_buffer_to_attr_keys();
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
                        return Ok(Some(self.builder.build()));
                    },
                    _ => {
                        self.state = States::AttributeValueUnquoted;
                        self.stream.reconsume();
                    }
                }
            },
            States::AttributeValueDoubleQuoted => {
                match char {
                    b'"' => {
                        self.state = States::AfterAttributeValueQuoted;
                        self.builder.commit_buffer_to_attr_value();
                    },
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
            States::AttributeValueUnquoted => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => self.state = States::BeforeAttributeName,
                    b'&' => {
                        self.return_state = States::AttributeValueUnquoted;
                        self.state = States::CharacterReference;
                    },
                    b'>' => {
                        self.state = States::Data;
                        self.builder.commit_buffer_to_attr_value();
                        return Ok(Some(self.builder.build()));
                    },
                    b'"' |
                    b'\''|
                    b'<' |
                    b'=' |
                    b'`' => {
                        // TODO: unexpected-character-in-unquoted-attribute-value error
                        self.builder.push_to_buffer(char);
                    },
                    _ => { self.builder.push_to_buffer(char); }
                }
            },
            States::AfterAttributeValueQuoted => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => self.state = States::BeforeAttributeName,
                    b'/' => self.state = States::SelfClosingStartTag,
                    b'>' => {
                        self.state = States::Data;
                        return Ok(Some(self.builder.build()));
                    },
                    _ => {
                        // TODO: missing-whitespace-between-attributes error
                        self.state = States::BeforeAttributeName;
                        self.stream.reconsume();
                    }
                }
            }

            // AfterAttributeValueQuoted,
            // SelfClosingStartTag,
            // BogusComment,
            States::MarkupDeclarationOpen => {
                match char {
                    b'-' => {
                        if self.stream.expect("-") {
                            self.stream.consume("--");
                            self.state = States::Comment;
                        } else {
                            todo!()
                        }
                    },
                    b'd' | b'D' => {
                        if self.stream.expect("OCTYPE") {
                            self.stream.consume("doctype");
                            self.state = States::DocType;
                        } else {
                            todo!()
                        }
                    },
                    b'[' => {
                        if self.stream.expect("CDATA[") {
                            todo!()
                        } else {
                            todo!()
                        }
                    },
                    _ => { todo!() }
                }
            },
            States::DocType => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => self.state = States::BeforeDocTypeName,
                    b'>' => {
                        self.stream.reconsume();
                        self.state = States::BeforeDocTypeName;
                    },
                    _ => {
                        // TODO: missing-whitesspace-before-doctype-name error
                        self.stream.reconsume();
                        self.state = States::BeforeDocTypeName;
                    }
                }
            },
            States::BeforeDocTypeName => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => { /* ignore */  },
                    b'A'..=b'Z' => {
                        self.builder.push_to_buffer(char);
                        self.state = States::DocTypeName;
                    },
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        todo!()
                    },
                    b'>' => {
                        // TODO: missing-doctype-name error
                        self.builder.force_quirks();
                        self.state = States::Data;
                        return Ok(Some(self.builder.build()));
                    },
                    _ => {
                        self.builder.push_to_buffer(char);
                        self.state = States::DocTypeName;
                    }
                }

            },
            States::DocTypeName => {
                match char {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => self.state = States::AfterDocTypeName,
                    b'>' => {
                        self.state = States::Data;
                        self.builder.set_variant(TokenVariant::Doctype)?;
                        self.builder.commit_buffer_to_doctype_name();
                        return Ok(Some(self.builder.build()));
                    },
                    b'A'..=b'Z' => {
                        self.builder.push_to_buffer(char);
                    },
                    b'\0' => {
                        // TODO: unexpected-null-character error
                        self.builder.push_replacement_character_to_buffer();
                    },
                    _ => {
                        self.builder.push_to_buffer(char);
                    }
                }
            },
            // AfterDocTypeName,
            // AfterDocTypeNamePublicKeyword,
            // BeforeDocTypePublicIdentifier,
            // DocTypePublicIdentifierDoubleQuoted,
            // DocTypePublicIdentifierSingleQuoted,
            // AfterDocTypePublicIdentifier,
            // BetweenDocTypePublicSystemIdentifiers,
            // AfterDocTypeSystemKeyword,
            // BeforeDocTypeSystemIdentifier,
            // DocTypeSystemIdentifierDoubleQuoted,
            // DocTypeSystemIdentifierSingleQuoted,
            // AfterDocTypeSystemIdentifier,
            // BogusDocType,
            _ => todo!(),
        }
        Ok(None)
    }
}
