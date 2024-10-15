use rs_lisp::lexer::*;
use rs_lisp::parser::Parser;
use std::fs;
use std::io::{self, Write};

fn main() {
    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let mut input = input.trim().to_string();

        if input == ":q" {
            println!("Exiting LISPrs...");
            break;
        }

        if input.len() > 3 && input[0..2] == ":l".to_owned() {
            println!("Loading from file...");
            input = fs::read_to_string(input[3..].to_string())
                .unwrap()
                .to_string();
        }

        let mut lexer = Lexer::new(&input);
        let mut tokens: Vec<Token> = Vec::new();
        let mut braces: Vec<Token> = Vec::new();

        loop {
            match lexer.next_token() {
                Ok(Some(token)) => {
                    match token {
                        Token::LParen(_, _) => braces.push(token.clone()),
                        Token::RParen(_, _) => match braces.pop() {
                            Some(_) => (),
                            None => panic!("Unmatched brace {:?}", token),
                        },
                        _ => (),
                    };

                    tokens.push(token)
                }
                Ok(None) => break,
                Err(e) => {
                    println!("Error: {:?}", e,);
                    break;
                }
            }
        }

        match braces.pop() {
            Some(token) => panic!("Unmatched brace {:?}", token),
            None => (),
        }

        let result = Parser::parse(tokens);

        println!("{}", result.show());
        println!("{:?}", result);
    }
}
