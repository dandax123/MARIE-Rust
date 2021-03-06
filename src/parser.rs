use std::{collections::HashMap, io::BufRead};

use crate::lexer::Token;
pub fn parser(
    tokens: Vec<Token>,
    mut accum: i64,
    mut symbol_table: HashMap<String, i64>,
    function_table: HashMap<String, Vec<Token>>,
) -> i64 {
    let mut instruction_tokens = tokens.into_iter().filter(|x| match x {
        Token::Variable(_, _) => false,
        _ => true,
    });

    while let Some(instr) = instruction_tokens.next() {
        // println!("Current Instruction: {:?}", instr);
        match instr {
            Token::Input => {
                accum = take_input().expect("Unable to take input");
            }
            Token::Output => {
                println!("Output: {}", accum);
            }
            Token::Store(c) => match symbol_table.insert(c.to_string(), accum) {
                None => panic!("This variable does not exist {}", c),
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
            Token::Jump(c) => match function_table.get(&c) {
                Some(x) => {
                    accum = parser(
                        x.to_owned(),
                        accum,
                        symbol_table.clone(),
                        function_table.clone(),
                    )
                }
                _ => {}
            },
            Token::Function(_c, k) => {
                accum = parser(
                    k.to_vec(),
                    accum,
                    symbol_table.clone(),
                    function_table.clone(),
                );
            }
            Token::Halt => panic!("Done executing"),
            _ => (),
        }
    }
    accum
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
