use chess::{Board, ChessMove, Color, Error, MoveGen, Piece, ALL_SQUARES};
use std::str::FromStr;

#[allow(dead_code)]
pub struct Engine {
    pub board: Board,
}

#[allow(dead_code)]
fn evaluate_board(board: &Board, side: Color) -> i32 {
    let mut score = 0;

    for square in ALL_SQUARES {
        if let Some(piece) = board.piece_on(square) {
            let mut piece_score = 0;

            match piece {
                Piece::Pawn => piece_score = 1,
                Piece::Knight => piece_score = 3,
                Piece::Bishop => piece_score = 3,
                Piece::Rook => piece_score = 5,
                Piece::Queen => piece_score = 9,
                Piece::King => (),
            }

            if board.color_on(square).unwrap() == side {
                score += piece_score;
            } else {
                score -= piece_score;
            }
        }
    }
    score
}

impl Engine {
    #[allow(dead_code)]
    pub fn set_board(&mut self, fen: &str) -> Result<(), Error> {
        self.board = Board::from_str(fen)?;
        Ok(())
    }

    #[allow(dead_code)]
    pub fn get_move(&self) -> ChessMove {
        let mut best_move: ChessMove = ChessMove::default();
        let mut best_score = i32::MIN;

        let iterable: MoveGen = MoveGen::new_legal(&self.board);
        for legal_move in iterable {
            let board = self.board.make_move_new(legal_move);
            let score = evaluate_board(&board, self.board.side_to_move());

            if score > best_score {
                best_score = score;
                best_move = legal_move;
            }
        }

        best_move
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_evaluate_board_white() {
        let board = Board::default();
        let score = evaluate_board(&board, Color::White);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_evaluate_board_black() {
        let board = Board::default();
        let score = evaluate_board(&board, Color::Black);
        assert_eq!(score, 0);
    }

    #[test]
    fn test_set_board_fen() {
        let mut engine = Engine {
            board: Board::default(),
        };

        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = engine.set_board(&fen);
        assert!(result.is_ok());
    }

    #[test]
    fn test_get_move() {
        let mut engine = Engine {
            board: Board::default(),
        };

        let fen = String::from("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1");
        let result = engine.set_board(&fen);
        assert!(result.is_ok());

        let best_move = engine.get_move();
        assert_eq!("a2a3", best_move.to_string());
    }
}
