use rustmarie::{
    file::extract_lines_from_file,
    lexer::{lex, Token},
    parser::parser,
};

// use std::env;

fn main() {
    let filename = "examples/add.mas";
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let x = extract_lines_from_file(filename).expect("File does not exist file");
    let tokens: Vec<Token> = lex(x);
    let parsetree = parser(tokens);
}
