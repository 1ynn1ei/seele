use crate::html::tokens::{Tokens, Tag};
use crate::stream::Stream;

pub type TokenList<'stream> = Vec<Tokens<'stream>>;
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
    WorkingTokenCollision,
    WorkingTokenUnexpectedEmpty,
}

#[derive(Default)]
pub struct WorkingToken<'stream> {
    token: Option<Tokens<'stream>>,
    value: Vec<&'stream u8>
}

pub struct Tokenizer<'stream> {
    stream: Stream<'stream>,
    state: States,
    return_state: States,
    tokens: TokenList<'stream>,
    working_token: WorkingToken<'stream>,
}

impl<'stream> Tokenizer<'stream> {
    pub fn new(data: &'stream [u8]) -> Self {
        Self {
            stream: Stream::new(data),
            state: States::Data,
            return_state: States::Data,
            tokens: TokenList::new(),
            working_token: WorkingToken::default()
        }
    }

    pub fn make_tokens(&mut self) -> Result<TokenList<'stream>, TokenizerError> {
        let tokens : Vec<Tokens<'stream>> = TokenList::new();
        // check EOF before rest
        // if EOF, go into EOF handler. some states create errors
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

        Ok(tokens)
    }

    fn prepare_working_token(&mut self, token: Tokens) -> Result<(), TokenizerError> {
        match self.working_token.token {
            Some(_) => Err(TokenizerError::WorkingTokenCollision),
            None => {
                self.working_token.token = Some(token);
                Ok(())
            }
        }
    }

    fn clear_working_token(&mut self) -> Result<(), TokenizerError> {
        match self.working_token.token {
            Some(_) => {
                self.working_token.token = None;
                self.working_token.value = Vec::new();
                Ok(())
            },
            None => Err(TokenizerError::WorkingTokenUnexpectedEmpty)
        }
    }

    fn push_working_token(&mut self, char: &'stream u8) {
        self.working_token.value.push(char);
    }
    
    //https://html.spec.whatwg.org/multipage/parsing.html#data-state
    fn data_state(&mut self, char: &u8) -> Result<(), TokenizerError> {
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
                self.tokens.push(Tokens::Character(char));
            },
            _ => {
                self.tokens.push(Tokens::Character(char));
            }
        }
        Ok(())
    }
    //https://html.spec.whatwg.org/multipage/parsing.html#tag-open-state
    fn tag_open_state(&mut self, char: &u8) -> Result<(), TokenizerError> {
        self.stream.advance();
        match char {
            b'!' => todo!(),
            b'/' => todo!(),
            b'?' => todo!(),
            b'a'..=b'z' | b'A'..=b'Z' => {
                self.state = States::TagName;
                self.prepare_working_token(Tokens::StartTag(Tag::new()));
                self.stream.reconsume();
            },
            _ => {
                // TODO: invalid-first-character-of-tag-name error
                self.state = States::Data;
                self.tokens.push(Tokens::Character(&b'<'));
                self.stream.reconsume();
            }
        }
        Ok(())
    }

    fn end_tag_open_state(&mut self, char: &u8) -> Result<(), TokenizerError> {
        self.stream.advance();
        match char {
            b'a'..=b'z' | b'A'..=b'Z' => {
                self.state = States::TagName;
                self.prepare_working_token(Tokens::EndTag(Tag::new()));
                self.stream.reconsume();
            },
            b'>' => {
                // TODO: missing-end-tag-name error
                self.state = States::Data;
            },
            _ => {
                // TODO: invalid-first-character-of-tag-name error
                todo!()
            }
        }
        Ok(())
    }

    fn tag_name_state(&mut self, char: &u8) -> Result<(), TokenizerError> {
        self.stream.advance();
        match char {
            b'\t' |
            0x0A /* LF */ |
            0x0C /* FF */ |
            b' ' => todo!(),
            b'/' => todo!(),
            b'>' => todo!(),
            b'A'..=b'Z' => todo!(),
            b'\0' => todo!(),
            _ => todo!(),
        }
        Ok(())
    }
}
