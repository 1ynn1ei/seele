mod html;
mod stream;
use html::tokenizer::Tokenizer;
use std::{env, fs};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let data = fs::read("./simple_test.html").unwrap();
    let mut tokenizer = Tokenizer::new(&data);
    match tokenizer.make_tokens() {
        Ok(res) => {
            for token in res {
                println!("{:?}", token);
            }
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
