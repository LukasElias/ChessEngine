#[derive(Debug, Clone, Copy, PartialEq)]
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

impl Piece {
    pub fn is_white_piece(&self) -> bool {
        match self {
            Piece::WhiteKing => true,
            Piece::WhiteQueen => true,
            Piece::WhiteRook => true,
            Piece::WhiteBishop => true,
            Piece::WhiteKnight => true,
            Piece::WhitePawn => true,
            _ => false,
        }
    }

    pub fn is_black_piece(&self) -> bool {
        match self {
            Piece::BlackKing => true,
            Piece::BlackQueen => true,
            Piece::BlackRook => true,
            Piece::BlackBishop => true,
            Piece::BlackKnight => true,
            Piece::BlackPawn => true,
            _ => false,
        }
    }
}

pub enum CastleType {
    Kingside,
    Queenside,
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

pub struct ChessMove {
    from: (usize, usize), // (row, column) of the piece's current position
    to: (usize, usize),   // (row, column) of the piece's destination position
    promotion: Option<Piece>, // Optional piece for pawn promotion
    capture: Option<Piece>,   // Optional captured piece
    en_passant: bool,          // Indicates if the move is an en passant capture
    castling: Option<CastleType>, // Indicates if the move is a castling move
}

#[derive(Debug)]
pub struct ChessBoard {
    pub board: [[Square; 8]; 8],
    pub is_white: bool,
    pub king_has_moved: bool,
    pub queen_rook_has_moved: bool,
    pub king_rook_has_moved: bool,
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

        ChessBoard { board, is_white: true, king_has_moved: false, queen_rook_has_moved: false, king_rook_has_moved: false }
    }

    pub fn available_moves(&self) -> Vec<ChessMove> {
        let mut available_moves: Vec<ChessMove> = vec![];
        if self.is_white {
            for y in 0..8 {
                for x in 0..8 {
                    if self.board[y][x].piece.is_white_piece() {
                        match self.board[y][x].piece {
                            Piece::WhiteKing => {
                                for relative_y in 0..3 {
                                    for relative_x in 0..3 {
                                        if relative_x == 1 && relative_y == 1 || (x as isize + relative_x as isize - 1) < 0 || (x + relative_x - 1) > 7 || (y as isize + relative_y as isize - 1) < 0 || (y + relative_y - 1) > 7 {
                                            continue; // check if relative_x and relative_y is valid
                                        }

                                        if self.board[y + relative_y - 1][x + relative_x - 1].piece.is_black_piece() {
                                            available_moves.push(
                                                ChessMove {
                                                    from: (x, y),
                                                    to: (x + relative_x - 1, y + relative_y - 1),
                                                    promotion: Option::None,
                                                    capture: Option::None,
                                                    en_passant: false,
                                                    castling: Option::None,
                                                }
                                            );
                                        }
                                    }
                                }

                                if !self.king_has_moved {
                                    if !self.king_rook_has_moved {
                                        if self.board[y][x + 1].piece == Piece::Empty && self.board[y][x + 2].piece == Piece::Empty {
                                            available_moves.push(
                                                ChessMove {
                                                    from: (x, y),
                                                    to: (x + 2, y),
                                                    promotion: Option::None,
                                                    capture: Option::None,
                                                    en_passant: false,
                                                    castling: Some(CastleType::Kingside),
                                                }
                                            )
                                        }
                                    }
                                    if !self.queen_rook_has_moved {
                                        if self.board[y][x - 1].piece == Piece::Empty && self.board[y][x - 2].piece == Piece::Empty && self.board[y][x - 3].piece == Piece::Empty {
                                            available_moves.push(
                                                ChessMove {
                                                    from: (x, y),
                                                    to: (x - 3, y),
                                                    promotion: Option::None,
                                                    capture: Option::None,
                                                    en_passant: false,
                                                    castling: Some(CastleType::Kingside),
                                                }
                                            )
                                        }
                                    }
                                }
                            },
                            Piece::WhiteQueen => {
                                
                            },
                            Piece::WhiteRook => {

                            },
                            Piece::WhiteBishop => {

                            },
                            Piece::WhiteKnight => {

                            },
                            Piece::WhitePawn => {

                            },
                            _ => (),
                        }
                    }
                }
            }
        }

        vec![]
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