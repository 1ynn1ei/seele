use crate::{
    arena::ArenaRef,
    html::{
        HTMLError,
        tokens::Token,
        dom,
    }
};

fn cmp_token_string(token_string: &[&u8], cmp: &str) -> bool {
    if token_string.len() != cmp.len() {
        false
    } else {
        for (i, val) in cmp.as_bytes().iter().enumerate() {
            if val != token_string[i] {
                return false
            }
        }
        true
    }
}

fn dom_string_from_token_string(token_string: &[&u8]) -> String {
    String::from_utf8(token_string
        .iter()
        .map(|&x| *x)
        .collect()).unwrap()
}

#[derive(Debug)]
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
    open_elements: Vec<ArenaRef>,
    dom_tree: dom::DomTree,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            template_insertion_modes: vec![Mode::Initial],
            open_elements: Vec::new(),
            dom_tree: dom::DomTree::new(dom::Document::new()),
        }
    }

    pub fn parse_token(&mut self, token: Token) -> Result<(), HTMLError> {
        if let Some(state) = self.template_insertion_modes.pop() {
            println!("[PARSER STATE: {:?}]", state);
            match state {
                Mode::Initial => self.initial_ruleset(token),
                _ => todo!()
            }
        } else {
            Err(HTMLError::ParserWithoutInsertionMode)
        }
    }

    fn initial_ruleset(&mut self, token: Token) -> Result<(), HTMLError> {
        match token {
            Token::Character(byte) => { todo!() },
            Token::Comment(bytes) => { todo!() },
            Token::Doctype(doctype) => {
                // make a new doc type node
                if cmp_token_string(&doctype.name, "html") {
                    self.dom_tree.insert(
                        dom::DocumentType::new(
                            dom_string_from_token_string(&doctype.name),
                            None,
                            None 
                        ), 0
                    )?;
                    self.template_insertion_modes.push(Mode::BeforeHtml);
                    Ok(())
                } else {
                    // parser error
                    Err(HTMLError::ParseError)
                }
            },
            _ => { todo!() },
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
