#[derive(Debug, Clone, Copy)]
pub enum Piece {
    King,
    Queen,
    Rook,
    Bishop,
    Knight,
    Pawn,
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    piece: Piece,
    color: Color,
}

#[derive(Debug)]
pub struct ChessBoard {
    board: [[Square; 8]; 8],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut board = [[Square { piece: Piece::Empty, color: Color::White }; 8]; 8];

        board[0] = [
            Square { piece: Piece::Rook, color: Color::Black },
            Square { piece: Piece::Knight, color: Color::Black },
            Square { piece: Piece::Bishop, color: Color::Black },
            Square { piece: Piece::Queen, color: Color::Black },
            Square { piece: Piece::King, color: Color::Black },
            Square { piece: Piece::Bishop, color: Color::Black },
            Square { piece: Piece::Knight, color: Color::Black },
            Square { piece: Piece::Rook, color: Color::Black },
        ];

        board[1] = [Square { piece: Piece::Pawn, color: Color::Black }; 8];

        board[6] = [Square { piece: Piece::Pawn, color: Color::White }; 8];

        board[7] = [
            Square { piece: Piece::Rook, color: Color::White },
            Square { piece: Piece::Knight, color: Color::White },
            Square { piece: Piece::Bishop, color: Color::White },
            Square { piece: Piece::Queen, color: Color::White },
            Square { piece: Piece::King, color: Color::White },
            Square { piece: Piece::Bishop, color: Color::White },
            Square { piece: Piece::Knight, color: Color::White },
            Square { piece: Piece::Rook, color: Color::White },
        ];

        ChessBoard { board }
    }
}