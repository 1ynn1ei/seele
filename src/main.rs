mod html;
mod stream;
use html::tokenizer::Tokenizer;
fn main() {
    let data = "<div>".as_bytes();
    let mut tokenizer = Tokenizer::new(data);
    match tokenizer.make_tokens() {
        Ok(res) => {
            println!("{:?}", res);
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
