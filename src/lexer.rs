use core::panic;
use std::num::ParseIntError;
#[derive(Debug, Clone, Copy)]
pub enum Conditions {
    Equal,
    Greater,
    Less,
}
#[derive(Debug, Clone)]

pub enum Token {
    Clear,
    Add(String),
    Subt(String),
    Load(String),
    Store(String),
    Input,
    Output,
    Skipcond(Conditions),
    Halt,
    Variable(String, i64),
    Jump(String),
    Function(String, Box<Vec<Token>>),
}

fn convert_hex_to_dec(raw: &str) -> Result<i64, ParseIntError> {
    let without_prefix = raw.trim_start_matches("0x");
    let z = i64::from_str_radix(without_prefix, 16)?;
    Ok(z)
}

fn extract_argument(x: Option<&&str>, ops: &str) -> String {
    match x {
        Some(&x) => x.to_string(),
        _ => panic!("{} requires an additional argument !!!", ops),
    }
}
fn parse_number(val: Option<&str>, v_type: &str) -> i64 {
    match val {
        Some(val) => {
            if v_type == "hex" {
                let k = convert_hex_to_dec(&val[..]).expect("Invalid Hex number");
                k
            } else {
                let i = &val[..].parse::<i64>().expect("Invalid Decimal Number");
                *i
            }
        }
        _ => panic!("variable requires a number"),
    }
}

fn extract_function_name(x: &str) -> String {
    x.split(",")
        .nth(0)
        .expect("A function must be named")
        .to_uppercase()
}
fn calculate_fn_length(x: &str) -> (String, usize) {
    let x: Vec<&str> = x.split(" ").take_while(|x| !x.contains(",")).collect();
    (x.join(" "), x.len())
}
pub fn lex(mut words: String, depth: i32) -> Vec<Token> {
    let mut ast = Vec::new();
    let words_clone = words.clone();
    let mut words = words.split(" ").peekable();
    let mut in_function = false;
    // println!("{:?}", words);
    while let Some(word) = words.next() {
        match &word.to_uppercase()[..] {
            "" => continue,
            "INPUT" => ast.push(Token::Input),
            "OUTPUT" => ast.push(Token::Output),
            "HALT" => ast.push(Token::Halt),
            "CLEAR" => ast.push(Token::Clear),
            x if x.contains("SKIPCOND") => {
                if let Some(&x) = words.peek() {
                    match x {
                        "000" => ast.push(Token::Skipcond(Conditions::Less)),
                        "400" => ast.push(Token::Skipcond(Conditions::Equal)),
                        "800" => ast.push(Token::Skipcond(Conditions::Greater)),
                        y=> panic!(
                            "Invalid Argument given ({}) to  Skipcond !!! USAGE Skipcond [000, 800, 400]", y
                        ),
                    }
                    words.nth(0);
                } else {
                    panic!("Provide an argument to skipcond");
                }
            }
            x if x.contains("ADD") => {
                ast.push(Token::Add(extract_argument(words.peek(), "ADD")));
                words.nth(0);
            }
            x if x.contains("STORE") => {
                ast.push(Token::Store(extract_argument(words.peek(), "STORE")));
                words.nth(0);
            }
            x if x.contains("SUBT") => {
                ast.push(Token::Subt(extract_argument(words.peek(), "SUBT")));
                words.nth(0);
            }
            x if x.contains("LOAD") => {
                ast.push(Token::Load(extract_argument(words.peek(), "LOAD")));
                words.nth(0);
            }
            x if x.contains("JUMP") => {
                ast.push(Token::Jump(extract_argument(words.peek(), "JUMP")));
                words.nth(0);
            }
            x if x.contains("ORG") => continue,
            x if x.contains(",") => {
                if let Some(&y) = words.peek() {
                    match &y.to_uppercase()[..] {
                        "DEC" => {
                            ast.push(Token::Variable(
                                extract_function_name(x),
                                parse_number(words.nth(1), "dec"),
                            ));
                        }
                        "HEX" => {
                            ast.push(Token::Variable(
                                extract_function_name(x),
                                parse_number(words.nth(1), "hex"),
                            ));
                        }
                        y => {
                            let fn_begin = words_clone.clone().to_uppercase().find(x);
                            match fn_begin {
                                Some(i) => {
                                    let remain = &words_clone[(i + x.len() + 1)..];
                                    let (funtion_string, skip_length) = calculate_fn_length(remain);
                                    // println!(" {} {:?} {} ", remain, funtion_string, skip_length);
                                    let t = lex(funtion_string, 1);
                                    ast.push(Token::Function(
                                        extract_function_name(x),
                                        Box::new(t),
                                    ));
                                    words.nth(skip_length - 1);
                                }
                                _ => panic!("impossible"),
                            }
                            // function world
                        }
                    }
                } else {
                    panic!("Functions or variables require arguments");
                }
            }

            x => println!("Unknown token here {}", x),
        }
        if let Some(&x) = words.peek() {
            if x.contains(",") && depth == 1 {
                return ast;
            }
        } else {
            continue;
        }
    }
    println!("{:?}", ast);
    ast
}
