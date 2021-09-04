use rustmarie::{
    file::extract_lines_from_file,
    lexer::{lex, Token},
    parser::parser,
};

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage cargo run [FILENAME]");
    let x = extract_lines_from_file(&filename[..]).expect("File does not exist file");
    let tokens: Vec<Token> = lex(x);
    parser(tokens);
}
