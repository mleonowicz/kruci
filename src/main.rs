mod engine;
mod parser;

use chess::{Board, ChessMove};
use parser::{Rule, UCIParser};
use pest::Parser;
use std::io;
use std::str::FromStr;
use std::sync::mpsc::channel;
use std::thread;

enum EngineCommand {
    Go,
    Position { board: Board },
    Quit,
}

fn main() {
    let (tx, rx) = channel::<EngineCommand>();

    let eng = thread::spawn(move || {
        let mut engine = engine::Engine {
            board: Board::default(),
        };

        loop {
            let value = rx.recv().unwrap();
            match value {
                EngineCommand::Position { board } => {
                    engine.board = board;
                    eprintln!("Engine: board set to {}", engine.board);
                }
                EngineCommand::Go => {
                    println!("bestmove {}", engine.get_move());
                }
                EngineCommand::Quit => break,
            }
        }
    });

    'outer: loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        let pairs = UCIParser::parse(Rule::command, &input).unwrap();
        for pair in pairs {
            match pair.as_rule() {
                Rule::uci => println!("uciok"),
                Rule::isready => print!("readyok"),
                Rule::quit => {
                    let _ = tx.send(EngineCommand::Quit);
                    break 'outer;
                }
                Rule::go => {
                    let _ = tx.send(EngineCommand::Go);
                }
                Rule::position => {
                    let mut moves: Vec<ChessMove> = Vec::new();
                    let mut board = Board::default();

                    for inner in pair.into_inner() {
                        match inner.as_rule() {
                            Rule::position_fen => {
                                let fen_string = inner.as_span().as_str();
                                board = Board::from_str(fen_string).unwrap();
                            }
                            Rule::position_startpos => {
                                board = Board::default();
                            }
                            Rule::chess_move => {
                                let m = ChessMove::from_str(inner.as_span().as_str()).unwrap();
                                moves.push(m);
                            }
                            _ => panic!(),
                        }
                    }

                    for m in moves {
                        let mut result = Board::default();
                        board.make_move(m, &mut result);
                        board = result;
                    }

                    let _ = tx.send(EngineCommand::Position { board });
                }
                _ => eprintln!("Command not supported: {}", pair),
            }
        }
    }

    eng.join().unwrap();
}
