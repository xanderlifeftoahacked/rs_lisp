use rs_lisp::evaluator::Evaluator;
use rs_lisp::parser::Parser;
use rs_lisp::{evaluator, lexer::*, lisptype::LispType};
use std::io::{self, Write};
use std::{env, fs};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let mut evaluator = Evaluator::new();

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

        let mut error = false;
        loop {
            match lexer.next_token() {
                Ok(Some(token)) => {
                    match token {
                        Token::LParen(_, _) => braces.push(token.clone()),
                        Token::RParen(_, _) => match braces.pop() {
                            Some(_) => (),
                            None => {
                                eprintln!("Unmatched brace {:?}", token);
                                error = true;
                                break;
                            }
                        },
                        _ => (),
                    };

                    tokens.push(token)
                }
                Ok(None) => break,
                Err(e) => {
                    eprintln!("Error: {:?}", e,);
                    break;
                }
            }
        }
        match braces.pop() {
            Some(token) => {
                eprintln!("Unmatched brace {:?}", token);
                error = true;
            }
            None => (),
        }

        if error {
            continue;
        }

        let result = Parser::parse(tokens);

        println!("{}", result.show());
        println!("{:?}", result);

        let eval_result = evaluator.eval(LispType::Cons(result));

        match eval_result {
            Ok(result) => println!("{}", result.show()),
            Err(e) => eprintln!("Evaluation error: {}", e),
        }
    }
}
