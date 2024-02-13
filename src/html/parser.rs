use crate::html::{
    tokens::Token,
    dom
};

pub enum Mode {
    Initial,
    BeforeHtml,
    BeforeHead,
    InHead,
    InHeadNoscript,
    AfterHead,
    InBody,
    Text,
    InTable,
    InTableText,
    InCaption,
    InColumnGroup,
    InTableBody,
    InRow,
    InCell,
    InSelect,
    InSelectInTable,
    InTemplate,
    AfterBody,
    InFrameset,
    AfterFrameset,
    AfterAfterBody,
    AfterAfterFrameset,
}

pub struct Parser {
    template_insertion_modes: Vec<Mode>,
    open_elements: Vec<Box<dyn dom::DomObject>>,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            template_insertion_modes: vec![Mode::Initial],
            open_elements: Vec::new(),
        }
    }

    pub fn parse_token(&mut self, token: Token) {
    }

    fn initial_ruleset(&mut self, token: Token) {
        match token {
            Token::Character(byte) => {},
            Token::Comment(chars) => {},
            Token::Doctype(doctype) => {},
            _ => {},
        }
    }
}
