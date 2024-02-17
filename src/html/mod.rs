pub mod tokens;
pub mod dom;
pub mod parser;
pub mod tokenizer;

#[derive(Debug)]
pub enum HTMLError {
    TokenBuilderImproperlyCleared,
    InaccessibleDomTreeNode
}

pub fn make_dom(data: &Vec<u8>) -> Result<(), HTMLError> {
    let mut tokenizer = tokenizer::Tokenizer::new(data);
    let mut parser = parser::Parser::new();
    loop {
        let next_emit : Option<tokens::Token>  = tokenizer.get_next_token()?;
        if let Some(token) = next_emit {
            match token {
                tokens::Token::EndOfFile => break,
                _ => {
                    println!("{:?}", token.present());
                    parser.parse_token(token)
                }
            }
        }
    }
    Ok(())
}
