use rs_lisp::conslist::*;
use rs_lisp::lexer::Lexer;
use rs_lisp::lisptype::*;
use std::fs;
use std::io::{self, Write};
use std::rc::Rc;

fn main() {
    let source: String = fs::read_to_string("text.lisp").unwrap();
    let mut lexer = Lexer::new(&source);

    loop {
        match lexer.next_token() {
            Ok(Some(token)) => println!("{:?}", token),
            Ok(None) => break,
            Err(e) => {
                println!("Error: {:?} at {}", e, lexer.get_position());
                break;
            }
        }
    }

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
