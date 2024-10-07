use std::io::{self, Write};
fn main() {
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
