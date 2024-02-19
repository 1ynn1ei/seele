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
    // template_insertion_modes: Vec<Mode>,
    insertion_mode: Mode,
    open_elements: Vec<ArenaRef>,
    dom_tree: dom::DomTree,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            // template_insertion_modes: vec![Mode::Initial],
            insertion_mode: Mode::Initial,
            open_elements: Vec::new(),
            dom_tree: dom::DomTree::new(dom::Document::new()),
        }
    }

    pub fn parse_token(&mut self, token: Token) -> Result<(), HTMLError> {
        // if let Some(state) = self.template_insertion_modes.pop() {
        println!("[PARSER STATE: {:?}]", self.insertion_mode);
        match self.insertion_mode {
            Mode::Initial => self.initial_ruleset(token),
            Mode::BeforeHtml => self.before_html_ruleset(token),
            Mode::BeforeHead => self.before_head_ruleset(token),
            Mode::InHead => self.in_head_ruleset(token),
            _ => todo!()
        }
        // } else {
        //     Err(HTMLError::ParserWithoutInsertionMode)
        // }
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
                    // self.template_insertion_modes.push(Mode::BeforeHtml);
                    self.insertion_mode = Mode::BeforeHtml;
                    Ok(())
                } else {
                    // parser error
                    Err(HTMLError::ParseError)
                }
            },
            _ => { todo!() },
        }
    }

    fn before_html_ruleset(&mut self, token: Token) -> Result<(), HTMLError> {
        match token {
            Token::Doctype(_) => {
                Err(HTMLError::ParseError)
            },
            Token::Comment(bytes) => {
                todo!()
            },
            Token::Character(byte) => {
                match byte {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => Ok(()),
                    _ => {
                        todo!()
                    }
                }
            },
            Token::StartTag(tag) => {
                if cmp_token_string(&tag.name, "html") {
                    self.dom_tree.insert(
                        dom::HeadElement::new(
                        ), 0
                    )?;
                    self.insertion_mode = Mode::BeforeHead;
                    Ok(())
                } else {
                    todo!()
                }
            },
            Token::EndTag(tag) => {
                todo!()
            },
            _ => {
                todo!()
            }
        }
    }

    fn before_head_ruleset(&mut self, token: Token) -> Result<(), HTMLError> {
        match token {
            Token::Character(byte) => {
                match byte {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => Ok(()),
                    _ => {
                        todo!()
                    }
                }
            },
            Token::Comment(bytes) => todo!(),
            Token::Doctype(_) => todo!(),
            Token::StartTag(tag) => {
                if cmp_token_string(&tag.name, "html") {
                    todo!()
                }
                if cmp_token_string(&tag.name, "head") {
                    let head_ref = self.dom_tree.insert(
                        dom::HeadElement::new(
                        ), 0
                    )?;
                    self.dom_tree.set_head(head_ref);
                    self.open_elements.push(head_ref);
                    self.insertion_mode = Mode::InHead;
                    Ok(())
                } else { todo!() }
            },
            _ => todo!()
        }
    }

    fn in_head_ruleset(&mut self, token: Token) -> Result<(), HTMLError> {
        match token {
            Token::Character(byte) => {
                match byte {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => Ok(()),
                    _ => {
                        todo!()
                    }
                }
            },
            Token::Comment(bytes) => todo!(),
            Token::Doctype(_) => todo!(),
            Token::StartTag(ref tag) => {
                match dom_string_from_token_string(&tag.name).as_str() {
                    "title" => {
                        self.generic_rcdata_element_ruleset(&token)
                    },
                    _ => todo!(),
                }
            },
            Token::EndTag(tag) => todo!(),
            _ => { todo!() }
        }
    }

    fn generic_rcdata_element_ruleset(&mut self, token: &Token) -> Result<(), HTMLError> {
        match token {
            Token::StartTag(tag) => {
                self.dom_tree.insert(
                    dom::Element::new(
                        dom_string_from_token_string(&tag.name),
                        "id".to_string(),
                        tag.get_class_list().unwrap_or_default(),
                    ), *self.open_elements.last().unwrap()
                );
            },
            _ => todo!(),
        }
        Ok(())
    }
    
    fn generic_raw_text_element_ruleset() -> Result<(), HTMLError> {
        todo!()
    }
}
