pub mod bot;

#[derive(Clone, Copy, Debug, Default)]
pub enum Piece {
    #[default]
    Empty,
    BlackPawn,
    BlackRook,
    BlackKnight,
    BlackBishop,
    BlackQueen,
    BlackKing,
    WhitePawn,
    WhiteRook,
    WhiteKnight,
    WhiteBishop,
    WhiteQueen,
    WhiteKing,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct Square {
    pub x: u8,
    pub y: u8,
}

#[derive(Clone, Copy, Debug, Default)]
pub struct ChessMove {
    // some stuff
}

#[derive(Clone, Copy, Debug)]
pub struct ChessPosition {
    pub pieces: [Piece; 64],
    pub is_white: bool,
    pub castling_availability: [bool; 4],
    pub en_passent: Option<Square>,
    pub half_moves: usize,
    pub full_moves: usize,
}

#[derive(Clone, Debug, Default)]
pub struct ChessMatch {
    pub move_history: Vec<ChessMove>,
    pub current_position: ChessPosition,
}

impl From<char> for Piece {
    fn from(value: char) -> Self {
        match value {
            'P' => Self::WhitePawn,
            'R' => Self::WhiteRook,
            'N' => Self::WhiteKnight,
            'B' => Self::WhiteBishop,
            'Q' => Self::WhiteQueen,
            'K' => Self::WhiteKing,
            'p' => Self::BlackPawn,
            'r' => Self::BlackRook,
            'n' => Self::BlackKnight,
            'b' => Self::BlackBishop,
            'q' => Self::BlackQueen,
            'k' => Self::BlackKing,
            _ => Self::Empty,
        }
    }
}

impl From<String> for Square {
    fn from(value: String) -> Self {
        Self {
            x: "abcdefgh".find(value.get(0..0).unwrap()).unwrap() as u8,
            y: u8::from_str_radix(value.get(1..1).unwrap(), 10).unwrap(),
        }
    }
}

impl ChessPosition {
    pub fn from_fen(fen: String) -> Self {
        Self {
            pieces: {
                let mut pieces: [Piece; 64] = [Piece::Empty; 64];

                let mut index: usize = 0;
                for rank in fen.splitn(8, "/") {
                    for char in rank.chars() {
                        if char.is_ascii_digit() {
                            index += char.to_digit(10).unwrap() as usize;
                        } else {
                            pieces[index] = Piece::from(char);
                            index += 1;
                        }
                    }
                }

                pieces
            },
            is_white: fen.split_whitespace().nth(1).unwrap() == "w",
            castling_availability: {
                let castle_string = fen.split_whitespace().nth(2).unwrap();
                [
                    castle_string.contains("K"),
                    castle_string.contains("Q"),
                    castle_string.contains("k"),
                    castle_string.contains("q"),
                ]
            },
            en_passent: {
                let en_passent = fen.split_whitespace().nth(3).unwrap();
                
                if en_passent == "-" {
                    None
                } else {
                    Some(Square::from(en_passent.to_string()))
                }
            },
            half_moves: usize::from_str_radix(fen.split_whitespace().nth(4).unwrap(), 10).unwrap(),
            full_moves: usize::from_str_radix(fen.split_whitespace().nth(5).unwrap(), 10).unwrap(),
        }
    }
}

impl Default for ChessPosition {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1".to_string())
    }
}
