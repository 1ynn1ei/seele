use crate::html::tokens::Tokens;
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
    CharacterReference,
}
#[derive(Debug)]
pub enum TokenizerError {
    WorkingTokenCollision,
    WorkingTokenUnexpectedEmpty,
}

pub struct Tokenizer<'stream> {
    stream: Stream<'stream>,
    state: States,
    return_state: States,
    tokens: TokenList<'stream>,
    working_token: Vec<&'stream u8>, 
}

impl<'stream> Tokenizer<'stream> {
    pub fn new(data: &'stream [u8]) -> Self {
        Self {
            stream: Stream::new(data),
            state: States::Data,
            return_state: States::Data,
            tokens: TokenList::new(),
            working_token: Vec::new(),
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

    fn prepare_working_token(&mut self) -> Result<(), TokenizerError> {
        if self.working_token.len() > 0 {
            Err(TokenizerError::WorkingTokenCollision)
        } else {
            Ok(())
        }
    }

    fn clear_working_token(&mut self) -> Result<(), TokenizerError> {
        if self.working_token.len() > 0 {
            self.working_token = Vec::new();
            Ok(())
        } else {
            Err(TokenizerError::WorkingTokenUnexpectedEmpty)
        }
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

    fn tag_open_state(&mut self, char: &u8) -> Result<(), TokenizerError> {
        self.stream.advance();
        match char {
            b'!' => todo!(),
            b'/' => todo!(),
            b'?' => todo!(),
            b'a'..=b'z' | b'A'..=b'Z' => {
                self.state = States::TagName;
                self.prepare_working_token();
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
