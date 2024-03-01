use crate::{
    arena::ArenaRef,
    html::{
        tokenizer,
        HTMLError,
        tokens::Token,
        dom,
        dom::{DomObject, DomTree}
    }
};
use std::mem;

type TokenizerState = tokenizer::States;
type ParserResult = Result<Option<TokenizerState>, HTMLError>;

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

fn byte_is_whitespacish(byte: &u8) -> bool {
    match byte {
        b'\t' |
        b'\n'/* LF */ |
        0x0C /* FF */ |
        b' ' => true,
        _ => false,
    }
}

#[derive(Debug, Clone, Copy)]
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
    original_mode: Option<Mode>,
    open_elements: Vec<ArenaRef>,
    last_element: ArenaRef,
    dom_tree: DomTree,
}

impl Parser {
    pub fn new() -> Self {
        Self {
            // template_insertion_modes: vec![Mode::Initial],
            insertion_mode: Mode::Initial,
            original_mode: None,
            open_elements: vec![0], 
            last_element: 0,
            dom_tree: DomTree::new(DomObject::Document),
        }
    }

    pub fn parse_token(&mut self, token: Token) -> ParserResult {
        // if let Some(state) = self.template_insertion_modes.pop() {
        println!("[PARSER STATE: {:?}]", self.insertion_mode);
        match self.insertion_mode {
            Mode::Initial => self.initial_ruleset(token),
            Mode::BeforeHtml => self.before_html_ruleset(token),
            Mode::BeforeHead => self.before_head_ruleset(token),
            Mode::InHead => self.in_head_ruleset(token),
            Mode::InHeadNoscript => todo!(),
            Mode::AfterHead => self.after_head_ruleset(token),
            Mode::InBody => self.in_body_ruleset(token),
            Mode::Text => self.in_text_ruleset(token),
            Mode::AfterBody => self.after_body_ruleset(token),
            Mode::AfterAfterBody => self.after_after_body_ruleset(token),
            _ => todo!()
        }
        // } else {
        //     Err(HTMLError::ParserWithoutInsertionMode)
        // }
    }

    fn initial_ruleset(&mut self, token: Token) -> ParserResult {
        match token {
            Token::Character(byte) => { todo!() },
            Token::Comment(bytes) => { todo!() },
            Token::Doctype(doctype) => {
                // make a new doc type node
                if cmp_token_string(&doctype.name, "html") {
                    self.dom_tree.insert(
                        DomObject::DocumentType(
                            dom::DocumentType::new(
                                dom_string_from_token_string(&doctype.name),
                                None,
                                None 
                            )
                        ), 0
                    )?;
                    // self.template_insertion_modes.push(Mode::BeforeHtml);
                    self.insertion_mode = Mode::BeforeHtml;
                    Ok(None)
                } else {
                    // parser error
                    Err(HTMLError::ParseError)
                }
            },
            _ => { todo!() },
        }
    }

    fn before_html_ruleset(&mut self, token: Token) -> ParserResult {
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
                    b' ' => Ok(None),
                    _ => {
                        todo!()
                    }
                }
            },
            Token::StartTag(tag) => {
                if cmp_token_string(&tag.name, "html") {
                    let html_ref = self.dom_tree.insert(
                        DomObject::Element(String::from("html")), 0
                    )?;
                    self.open_elements.push(html_ref);
                    self.insertion_mode = Mode::BeforeHead;
                    Ok(None)
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

    fn before_head_ruleset(&mut self, token: Token) -> ParserResult {
        match token {
            Token::Character(byte) => {
                match byte {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => Ok(None),
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
                        DomObject::Head, 0
                    )?;
                    self.dom_tree.set_head(head_ref);
                    self.open_elements.push(head_ref);
                    self.insertion_mode = Mode::InHead;
                    Ok(None)
                } else { todo!() }
            },
            _ => todo!()
        }
    }

    fn in_head_ruleset(&mut self, token: Token) -> ParserResult {
        match token {
            Token::Character(byte) => {
                match byte {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => Ok(None),
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
            Token::EndTag(_) => {
                println!("{:?}", self.open_elements);
                self.open_elements.pop();
                self.insertion_mode = Mode::AfterHead;
                Ok(None)
            },
            _ => { todo!() }
        }
    }
    
    fn in_text_ruleset(&mut self, token: Token) -> ParserResult {
        match token {
            Token::Character(byte) => {
                self.insert_or_merge_text_into_tree(
                    String::from_utf8(vec![*byte]).unwrap()
                    )?;
                Ok(None)
            },
            Token::EndTag(tag) => {
                self.open_elements.pop();
                if let Some(original_mode) = self.original_mode {
                    self.insertion_mode = original_mode; 
                    Ok(None)
                } else {
                    Err(HTMLError::ParserLostOriginalMode)
                }
            },
            _ => todo!()
        }
    }

    fn after_head_ruleset(&mut self, token: Token) -> ParserResult {
        match token {
            Token::Character(byte) => {
                match byte {
                    b'\t' |
                    b'\n'/* LF */ |
                    0x0C /* FF */ |
                    b' ' => {
                        self.insert_or_merge_text_into_tree(
                            String::from_utf8(vec![*byte]).unwrap()
                            )?;
                        Ok(None)
                    },
                    _ => todo!(),
                }
            },
            Token::StartTag(tag) => {
                let tag_name = dom_string_from_token_string(&tag.name);
                match tag_name.as_str() {
                    "body" => {
                        let body_ref = self.dom_tree.insert(
                            DomObject::Element(
                                tag_name,
                            ), *self.open_elements.last().unwrap() // TODO: cleanup
                        )?;
                        self.open_elements.push(body_ref);
                        // TODO: frameset-ok = 'not ok'
                        self.insertion_mode = Mode::InBody;
                        Ok(None)
                    },
                    _ => todo!(),
                }
            },
            _ => todo!()
        }
    }

    fn in_body_ruleset(&mut self, token: Token) -> ParserResult {
        match token {
            Token::Character(byte) => {
                self.insert_or_merge_text_into_tree(
                    String::from_utf8(vec![*byte]).unwrap()
                    )?;
                Ok(None)
            },
            Token::EndTag(tag) => {
                let tag_name = dom_string_from_token_string(&tag.name);
                match tag_name.as_str() {
                    "body" => {
                        // TODO: check if body is in scope
                        // TODO: check rest of elements in stack
                        self.insertion_mode = Mode::AfterBody;
                        Ok(None)
                    },
                    _ => todo!()
                }
            },
            _ => todo!()
        }
    }

    fn after_body_ruleset(&mut self, token: Token) -> ParserResult {
        match token {
            Token::Character(byte) => {
                if byte_is_whitespacish(byte) {
                    self.in_body_ruleset(token)
                } else {
                    todo!()
                }
            },
            Token::EndTag(tag) => {
                let tag_name = dom_string_from_token_string(&tag.name);
                match tag_name.as_str() {
                    "html" => {
                        self.insertion_mode = Mode::AfterAfterBody;
                        Ok(None)
                    },
                    _ => todo!()
                }
            },
            _ => todo!()
        }
    }

    fn after_after_body_ruleset(&mut self, token: Token) -> ParserResult {
        match token {
            Token::Character(byte) => {
                if byte_is_whitespacish(byte) {
                    self.in_body_ruleset(token)
                } else {
                    todo!()
                }
            },
            _ => todo!()
        }
    }

    fn generic_rcdata_element_ruleset(&mut self, token: &Token) -> ParserResult {
        match token {
            Token::StartTag(tag) => {
                self.dom_tree.insert(
                    DomObject::Element(
                        dom_string_from_token_string(&tag.name),
                    ), *self.open_elements.last().unwrap() // TODO: cleanup
                );
            },
            _ => todo!(),
        }
        self.original_mode = Some(self.insertion_mode);
        self.insertion_mode = Mode::Text;
        Ok(Some(TokenizerState::RCData))
    }
    
    fn generic_raw_text_element_ruleset() -> Result<(), HTMLError> {
        todo!()
    }

    fn insert_into_tree(&mut self, obj: DomObject) -> Result<(), HTMLError> {
        if let Some(parent_ref) = self.open_elements.last() {
            let new_ref = self.dom_tree.insert(
                obj, *parent_ref
            )?;
            self.last_element = new_ref;
            Ok(())
        } else {
            Err(HTMLError::OrphanObject)
        }
    }

    fn insert_or_merge_text_into_tree(&mut self, data: String) -> Result<(), HTMLError> {
        if let Some(node) = self.dom_tree.arena.get_mut(self.last_element) {
            match node.dom_obj {
                DomObject::Text(ref mut string) => {
                    string.push_str(&data);
                    Ok(())
                },
                _ => {
                    self.insert_into_tree(DomObject::Text(data))
                }
            }
        } else {
            Err(HTMLError::OrphanObject)
        }
    }
}
