use chess::{Board, ALL_SQUARES, Color, Piece, MoveGen, ChessMove};

struct Engine {
    board: Board
}

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
    pub fn set_board(&mut self, fen: String) {
        self.board = Board::from_str(&fen)?;
    }

    pub fn get_move(self) -> String {
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

        best_move.to_string()
    }
}
