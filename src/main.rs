mod engine;
use chess::Board;
use pest::Parser;
use pest_derive::Parser;
use std::io;
use std::sync::mpsc::channel;
use std::thread;

#[derive(Parser)]
#[grammar = "../res/uci.pest"]
pub struct UCIParser;

fn main() {
    let (tx, rx) = channel::<String>();

    let eng = thread::spawn(move || {
        let engine = engine::Engine {
            board: Board::default(),
        };

        loop {
            let value = rx.recv().unwrap();
            match value.trim() {
                "go" => {
                    let best_move = engine.get_move();
                    println!("Engine: bestmove {}", best_move.to_string());
                }
                _ => println!("Engine: Not supported yet!"),
            }
        }
    });

    loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        if input.trim() == "exit" {
            break;
        }

        let result = UCIParser::parse(Rule::command, &input);
        match result {
            Ok(_) => tx.send(input).unwrap(),
            Err(e) => println!("Error: {}", e),
        }
    }

    eng.join().unwrap();
}
