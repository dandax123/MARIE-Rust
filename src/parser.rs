use std::{
    collections::HashMap,
    io::{BufRead, Read},
};

use crate::lexer::Token;
pub fn parser(tokens: Vec<Token>) {
    let mut accum: i64 = 0;
    let mut symbol_table: HashMap<String, i64> = tokens
        .clone()
        .into_iter()
        .filter_map(|d| match d {
            Token::Variable(c, i) => Some((c.to_string(), i)),
            _ => None,
        })
        .collect();
    let instruction_tokens: Vec<Token> = tokens
        .into_iter()
        .filter(|x| match x {
            Token::Variable(_, _) => false,
            _ => true,
        })
        .collect();
    for instr in instruction_tokens.into_iter() {
        match instr {
            Token::Input => {
                accum = take_input().expect("Unable to take input");
            }
            Token::Output => {
                println!("Output: {}", accum);
            }
            Token::Store(c) => match symbol_table.insert(c.to_string(), accum) {
                None => panic!("This variable does not exist"),
                _ => (),
            },
            Token::Load(c) => {
                let val = symbol_table
                    .get(&c.to_string())
                    .expect("Variable doesn't exist");
                accum = *val;
            }
            Token::Clear => accum = 0,
            Token::Add(c) => {
                let val = symbol_table
                    .get(&c.to_string())
                    .expect("Variable doesn't exist");
                accum += *val;
            }
            Token::Subt(c) => {
                let val = symbol_table
                    .get(&c.to_string())
                    .expect("Variable doesn't exist");
                accum -= *val;
            }
            Token::Halt => (),
            _ => (),
        }
    }
}

fn take_input() -> std::io::Result<i64> {
    let mut buffer = String::new();
    std::io::stdin().lock().read_line(&mut buffer)?;
    let num = buffer
        .lines()
        .nth(0)
        .unwrap()
        .parse::<i64>()
        .expect("Invalid Number");
    Ok(num)
}
