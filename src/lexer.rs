use core::panic;
use std::num::ParseIntError;
#[derive(Debug, Clone, Copy)]
pub enum Conditions {
    Equal,
    Greater,
    Less,
}
#[derive(Debug, Clone, Copy)]

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
    Variable(char, i64),
}

fn convert_hex_to_dec(raw: &str) -> Result<i64, ParseIntError> {
    let without_prefix = raw.trim_start_matches("0x");
    let z = i64::from_str_radix(without_prefix, 16)?;
    Ok(z)
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
fn extract_variable(token: &str, v_type: &str) -> (char, i64) {
    let tokens: Vec<String> = token.split(v_type).map(|x| x.trim().to_string()).collect();
    let val = tokens
        .get(1)
        .expect("Please provide a value for the variable");
    let char_name = tokens
        .get(0)
        .unwrap()
        .to_uppercase()
        .chars()
        .nth(0)
        .expect("A variable must be named");
    if v_type == "hex" {
        let k = convert_hex_to_dec(&val[..]).expect("Invalid Hex number");
        (char_name, k)
    } else {
        let i = &val[..].parse::<i64>().expect("Invalid Decimal Number");
        (char_name, *i)
    }
}
pub fn lex(lines: Vec<String>) -> Vec<Token> {
    let mut ast = Vec::new();
    let removed_comments = remove_comments(lines);
    for line in removed_comments.into_iter() {
        match &line.to_uppercase()[..] {
            "INPUT" => ast.push(Token::Input),
            "OUTPUT" => ast.push(Token::Output),
            "HALT" => ast.push(Token::Halt),
            "CLEAR" => ast.push(Token::Clear),
            x if x.contains("SKIPCOND") => {
                let c = extract_argument(x, "SKIPCOND");
                match c {
                    '0' => ast.push(Token::Skipcond(Conditions::Less)),
                    '4' => ast.push(Token::Skipcond(Conditions::Equal)),
                    '8' => ast.push(Token::Skipcond(Conditions::Greater)),
                    _ => panic!("Invalid Argument to Skipcond"),
                }
            }
            x if x.contains("ADD") => ast.push(Token::Add(extract_argument(x, "ADD"))),
            x if x.contains("STORE") => ast.push(Token::Store(extract_argument(x, "STORE"))),
            x if x.contains("SUBT") => ast.push(Token::Subt(extract_argument(x, "SUBT"))),
            x if x.contains("LOAD") => ast.push(Token::Load(extract_argument(x, "LOAD"))),
            x if x.contains("HEX") => {
                let (c, i) = extract_variable(&x.to_lowercase()[..], "hex");
                ast.push(Token::Variable(c, i))
            }
            x if x.contains("DEC") => {
                let (c, i) = extract_variable(&x.to_lowercase()[..], "dec");
                ast.push(Token::Variable(c, i))
            }
            x => println!("Unknown token {}", x),
        }
    }
    ast
}
