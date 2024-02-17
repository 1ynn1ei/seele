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
    document: dom::Document,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            template_insertion_modes: vec![Mode::Initial],
            open_elements: Vec::new(),
            document: dom::Document::default()
        }
    }

    pub fn parse_token(&mut self, token: Token) {
    }

    fn initial_ruleset(&mut self, token: Token) {
        match token {
            Token::Character(byte) => {},
            Token::Comment(bytes) => {},
            Token::Doctype(doctype) => {
                // make a new doc type node
            },
            _ => {},
        }
    }

    fn before_html_ruleset(&mut self, token: Token) {
        match token {
            Token::Doctype(_) => {
                // TODO: parse error
            },
            Token::Comment(bytes) => {
                todo!()
            },
            Token::Character(byte) => {
                todo!()
            },
            Token::StartTag(tag) => {
                todo!()
            },
            Token::EndTag(tag) => {
                todo!()
            },
            _ => {
                todo!()
            }
        }
    }
}
