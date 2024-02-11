pub mod tokens;
pub mod dom;
pub mod parser;
pub mod tokenizer;

#[derive(Debug)]
pub enum HTMLError {
    TokenBuilderImproperlyCleared
}
