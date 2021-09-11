use std::{collections::HashMap, io::BufRead};

use crate::lexer::Token;
pub fn parser(tokens: Vec<Token>, mut accum: i64, mut symbol_table: HashMap<String, i64>) {
    let mut instruction_tokens = tokens.into_iter().filter(|x| match x {
        Token::Variable(_, _) => false,
        _ => true,
    });
    while let Some(instr) = instruction_tokens.next() {
        match instr {
            Token::Input => {
                accum = take_input().expect("Unable to take input");
            }
            Token::Output => {
                println!("Output: {}", accum);
            }
            Token::Store(c) => match symbol_table.insert(c, accum) {
                None => panic!("This variable does not exist"),
                _ => (),
            },
            Token::Load(c) => {
                let val = symbol_table.get(&c).expect("Variable doesn't exist");
                accum = *val;
            }
            Token::Clear => accum = 0,
            Token::Add(c) => {
                let val = symbol_table.get(&c).expect("Variable doesn't exist");
                accum += *val;
            }
            Token::Subt(c) => {
                let val = symbol_table.get(&c).expect("Variable doesn't exist");
                accum -= *val;
            }
            Token::Skipcond(crate::lexer::Conditions::Less) => {
                if accum < 0 {
                    instruction_tokens.nth(0);
                }
            }
            Token::Skipcond(crate::lexer::Conditions::Equal) => {
                if accum == 0 {
                    instruction_tokens.nth(0);
                }
            }
            Token::Skipcond(crate::lexer::Conditions::Greater) => {
                if accum > 0 {
                    instruction_tokens.nth(0);
                }
            }
            // Token::Jump(c) => {}
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
