use core::panic;
#[derive(Debug)]
pub enum Conditions {
    Equal,
    Greater,
    Less,
}
#[derive(Debug)]
pub enum Token {
    Clear,
    Add(char),
    Subt(char),
    Load(char),
    Store(char),
    Input,
    Output,
    Skipcond(Conditions),
    Halt,
    Variable(char, char, i32),
}

fn remove_comments(lines: Vec<String>) -> Vec<String> {
    return lines
        .into_iter()
        .map(|x| x.split("/").nth(0).expect("msg").trim().to_string())
        .filter(|x| !x.is_empty())
        .collect();
}
fn extract_argument(x: &str, ops: &str) -> char {
    return x
        .split(ops)
        .nth(1)
        .unwrap()
        .trim()
        .to_string()
        .chars()
        .next()
        .expect(&format!("{} requires an additional argument !!!", ops)[..]);
}
pub(crate) fn lex(lines: Vec<String>) -> Vec<Token> {
    let mut ast = Vec::new();
    let removed_comments = remove_comments(lines);
    for line in removed_comments.into_iter() {
        match &line.to_uppercase()[..] {
            "INPUT" => ast.push(Token::Input),
            "OUTPUT" => ast.push(Token::Output),
            "HALT" => ast.push(Token::Halt),
            "CLEAR" => ast.push(Token::Clear),
            x if x.contains("ADD") => ast.push(Token::Add(extract_argument(x, "ADD"))),
            x if x.contains("STORE") => ast.push(Token::Store(extract_argument(x, "STORE"))),
            x if x.contains("SUBT") => ast.push(Token::Subt(extract_argument(x, "SUBT"))),
            x if x.contains("LOAD") => ast.push(Token::Load(extract_argument(x, "LOAD"))),
            x => println!("Unknown token {}", x),
        }
    }
    ast
}
