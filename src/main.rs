mod file;
mod lexer;
use file::extract_lines_from_file;
use lexer::lex;
// use std::env;

fn main() {
    let filename = "examples/add.mas";
    // let args: Vec<String> = env::args().collect();
    // println!("{:?}", args);
    let x = extract_lines_from_file(filename).expect("File does not exist file");

    for i in lex(x).into_iter() {
        println!("{:?}", i);
    }
}
