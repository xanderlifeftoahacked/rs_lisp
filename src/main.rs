use rs_lisp::lexer::*;
use rs_lisp::parser::Parser;
use std::fs;
use std::io::{self, Write};

fn main() {
    let source: String = fs::read_to_string("text.lisp").unwrap();
    let mut lexer = Lexer::new(&source);
    let mut tokens: Vec<Token> = Vec::new();

    loop {
        match lexer.next_token() {
            Ok(Some(token)) => tokens.push(token),
            Ok(None) => break,
            Err(e) => {
                println!("Error: {:?} at {}", e, lexer.get_position());
                break;
            }
        }
    }

    let result = Parser::parse(tokens);
    println!("{}", result.show());

    println!("{:?}", result);

    loop {
        print!(">>> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("Failed to read line!");

        let input = input.trim();

        if input == "exit" {
            println!("Exiting LISPrs...");
            break;
        }

        println!("{}", input);
    }
}
