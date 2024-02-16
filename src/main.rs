mod html;
mod stream;
mod arena;
use std::{env, fs};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let data = fs::read("./simple_test.html").unwrap();
    match html::make_dom(&data) {
        Ok(res) => {
        },
        Err(err) => {
            println!("{:?}", err);
        }
    }
}
