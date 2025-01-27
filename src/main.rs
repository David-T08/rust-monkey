use lexer::Lexer;
use std::{io, process};

fn main() {
		println!("Monkey v0.1.0 RLPL:");
		loop {
				let mut input = String::new();

				io::stdin().read_line(&mut input).unwrap();
				let input = input.trim();

				if input == "exit" {
						process::exit(0);
				}

				let lexer = Lexer::new(input.to_string());
				for token in lexer {
					println!("{:?}", token);
				}
		}
}
