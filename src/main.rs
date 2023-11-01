mod engine;
use pest::Parser;
use pest_derive::Parser;
use std::io;

#[derive(Parser)]
#[grammar = "../res/uci.pest"]
pub struct UCIParser;

fn main() {
    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let result = UCIParser::parse(Rule::command, &input);
        match result {
            Ok(_) => println!("Valid command: {:#?}", result),
            Err(e) => println!("Invalid command: {}", e),
        }
    }
}
