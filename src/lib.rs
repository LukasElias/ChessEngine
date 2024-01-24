#[derive(Debug, Clone, Copy)]
pub enum Piece {
    BlackKing,
    BlackQueen,
    BlackRook,
    BlackBishop,
    BlackKnight,
    BlackPawn,
    WhiteKing,
    WhiteQueen,
    WhiteRook,
    WhiteBishop,
    WhiteKnight,
    WhitePawn,
    Empty,
}

#[derive(Debug, Clone, Copy)]
pub struct Square {
    pub piece: Piece,
}

impl Square {
    pub fn to_emoji(&self) -> char {
        match self.piece {
            Piece::BlackKing => char::from_u32(0x2654).unwrap(), // ♔
            Piece::BlackQueen => char::from_u32(0x2655).unwrap(), // ♕
            Piece::BlackRook => char::from_u32(0x2656).unwrap(), // ♖
            Piece::BlackBishop => char::from_u32(0x2657).unwrap(), // ♗
            Piece::BlackKnight => char::from_u32(0x2658).unwrap(), // ♘
            Piece::BlackPawn => char::from_u32(0x2659).unwrap(), // ♙

            Piece::WhiteKing => char::from_u32(0x265A).unwrap(), // ♚
            Piece::WhiteQueen => char::from_u32(0x265B).unwrap(), // ♛
            Piece::WhiteRook => char::from_u32(0x265C).unwrap(), // ♜
            Piece::WhiteBishop => char::from_u32(0x265D).unwrap(), // ♝
            Piece::WhiteKnight => char::from_u32(0x265E).unwrap(), // ♞
            Piece::WhitePawn => char::from_u32(0x265F).unwrap(), // ♟
            Piece::Empty => char::from_u32(0x20).unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct ChessBoard {
    board: [[Square; 8]; 8],
}

impl ChessBoard {
    pub fn new() -> ChessBoard {
        let mut board: [[Square; 8]; 8] = [[Square { piece: Piece::Empty }; 8]; 8];

        board[0] = [
            Square { piece: Piece::BlackRook },
            Square { piece: Piece::BlackKnight },
            Square { piece: Piece::BlackBishop },
            Square { piece: Piece::BlackQueen },
            Square { piece: Piece::BlackKing },
            Square { piece: Piece::BlackBishop },
            Square { piece: Piece::BlackKnight },
            Square { piece: Piece::BlackRook },
        ];

        board[1] = [Square { piece: Piece::BlackPawn}; 8];

        board[6] = [Square { piece: Piece::WhitePawn }; 8];

        board[7] = [
            Square { piece: Piece::WhiteRook },
            Square { piece: Piece::WhiteKnight },
            Square { piece: Piece::WhiteBishop },
            Square { piece: Piece::WhiteQueen },
            Square { piece: Piece::WhiteKing },
            Square { piece: Piece::WhiteBishop },
            Square { piece: Piece::WhiteKnight },
            Square { piece: Piece::WhiteRook },
        ];

        ChessBoard { board }
    }
}

impl std::fmt::Display for ChessBoard {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut result = String::from("");

        for y in 0..8 {
            for x in 0..8 {
                if (x + y) % 2 == 0 {
                    result.push_str("\u{001b}[40;1m");
                } else {
                    result.push_str("\u{001b}[47m");
                }

                match self.board[y][x].piece {
                    Piece::WhiteKing => result.push_str("\u{001b}[37m"),
                    Piece::WhiteQueen => result.push_str("\u{001b}[37m"),
                    Piece::WhiteRook => result.push_str("\u{001b}[37m"),
                    Piece::WhiteBishop => result.push_str("\u{001b}[37m"),
                    Piece::WhiteKnight => result.push_str("\u{001b}[37m"),
                    Piece::WhitePawn => result.push_str("\u{001b}[37m"),
                    _ => result.push_str("\u{001b}[30m"),
                }
                result.push(self.board[y][x].to_emoji());
                result.push_str(" \u{001b}[0m");
            }

            result.push_str("\n");
        }

        write!(f, "{}", result)
    }
}