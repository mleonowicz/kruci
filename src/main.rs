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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parser_uci_simple() {
        let result = UCIParser::parse(Rule::command, "uci\n");
        assert!(result.is_ok());
        let result = UCIParser::parse(Rule::command, "debug on\n");
        assert!(result.is_ok());
        let result = UCIParser::parse(Rule::command, "debug off\n");
        assert!(result.is_ok());
        let result = UCIParser::parse(Rule::command, "isready\n");
        assert!(result.is_ok());
        let result = UCIParser::parse(Rule::command, "stop\n");
        assert!(result.is_ok());
        let result = UCIParser::parse(Rule::command, "quit\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_position() {
        let result = UCIParser::parse(Rule::command, "position startpos\n");
        assert!(result.is_ok());

        let result = UCIParser::parse(Rule::command, "position startpos moves a1a2\n");
        assert!(result.is_ok());

        let result = UCIParser::parse(Rule::command, "position startpos moves a1a2 a2a3\n");
        assert!(result.is_ok());

        let result = UCIParser::parse(
            Rule::command,
            "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1\n",
        );
        assert!(result.is_ok());

        let result = UCIParser::parse(
            Rule::command,
            "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves a1a2\n",
        );
        assert!(result.is_ok());

        let result = UCIParser::parse(Rule::command, "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves a1a2 a2a3\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_invalid_position() {
        let result = UCIParser::parse(Rule::command, "position\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "position moves\n");
        assert!(result.is_err());

        let result = UCIParser::parse(
            Rule::command,
            "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR\n",
        );
        assert!(result.is_err());

        let result = UCIParser::parse(
            Rule::command,
            "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR moves a1a2\n",
        );
        assert!(result.is_err());

        let result = UCIParser::parse(
            Rule::command,
            "position fen rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 moves\n",
        );
        assert!(result.is_err());
    }

    #[test]
    fn test_parser_go() {
        let result = UCIParser::parse(Rule::command, "go searchmoves a1a2 ponder wtime 1000 btime 1000 winc 1000 binc 1000 movestogo 5 depth 5 nodes 5 mate 5 movetime 1000 infinite\n");
        assert!(result.is_ok());

        // Decide later on this case, for now it's ok
        let result = UCIParser::parse(Rule::command, "go wtime 1000 wtime 1200\n");
        assert!(result.is_ok());
    }

    #[test]
    fn test_parser_invalid_go() {
        let result = UCIParser::parse(Rule::command, "go searchmoves\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go wtime\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go wtime\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go winc\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go binc\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go movestogo\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go depth\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go nodes\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go mates\n");
        assert!(result.is_err());

        let result = UCIParser::parse(Rule::command, "go movetime\n");
        assert!(result.is_err());
    }
}
