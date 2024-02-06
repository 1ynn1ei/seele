mod html;
use html::tokenizer::Tokenizer;
fn main() {
    let data = "<div>".as_bytes();
    let mut tokenizer = Tokenizer::new(data);
    tokenizer.make_tokens();
}
