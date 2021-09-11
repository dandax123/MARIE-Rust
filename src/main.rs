use rustmarie::{
    file::{extract_lines_from_file, extract_token},
    lexer::{lex, Token},
    parser::parser,
};

use std::{collections::HashMap, env};

fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = args.get(1).expect("Usage cargo run [FILENAME]");
    let x = extract_lines_from_file(&filename[..]).expect("File does not exist file");
    let tokens: Vec<Token> = lex(extract_token(x), 0);
    let symbol_table: HashMap<String, i64> = tokens
        .iter()
        .filter_map(|d| match d {
            Token::Variable(c, i) => Some((c.to_string(), *i)),
            _ => None,
        })
        .collect();

    // parser(tokens, 0, symbol_table);
}
