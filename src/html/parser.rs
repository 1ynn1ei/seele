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
}

impl Parser {
    pub fn new() -> Self {
        Self {
            template_insertion_modes: Vec::new()
        }
    }

    pub fn parse_token(&mut self, token: Token) {

    }
}
