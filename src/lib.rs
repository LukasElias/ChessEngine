const KING_OFFSETS: [(isize, isize);8] = [
    (-1, -1),
    (0, -1),
    (1, -1),
    (-1, 0),
    (1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
];

const KNIGHT_OFFSETS: [(isize, isize);8] = [
    (-2, -1),
    (-1, -2),
    (1, 2),
    (2, 1),
    (2, -1),
    (1, -2),
    (-1, -2),
    (-2, -1),
];

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

    pub fn from_char(char: char) -> Self {
        match char {
            'k' => Piece::BlackKing,
            'q' => Piece::BlackQueen,
            'r' => Piece::BlackRook,
            'b' => Piece::BlackBishop,
            'n' => Piece::BlackKnight,
            'p' => Piece::BlackPawn,
            'K' => Piece::WhiteKing,
            'Q' => Piece::WhiteQueen,
            'R' => Piece::WhiteRook,
            'B' => Piece::WhiteBishop,
            'N' => Piece::WhiteKnight,
            'P' => Piece::WhitePawn,
            _ => Piece::Empty,
        }
    }
}

pub struct ChessMove {
    pub notation: String,
    pub from: Pos,
    pub to: Pos,
    pub castle: Option<bool>, // if true it's a kingside castle otherwise it's a queenside castle
    pub en_passent: Option<Pos>,
    pub promotion: Option<Piece>,
}

#[derive(Debug, Clone, Copy)]
pub struct Pos {
    pub column: usize,
    pub row: usize,
}

impl Pos {
    pub fn to_chess_notation(&self) -> String {
        format!("{}{}", "abcdefhg".as_bytes()[self.column] - 1, self.row)
    }

    pub fn from_chess_notation(notation: String) -> Option<Self> {
        if notation.len() != 2 {
            return Option::None;
        }

        Some(Self {
            column: "abcdefhg".find(notation.chars().next().unwrap()).unwrap(),
            row: notation.chars().nth(1).unwrap().to_digit(10).unwrap() as usize,
        })
    }
}

#[derive(Debug, Clone, Copy)]
pub struct ChessBoard {
    pub board: [Piece; 64],
    pub turn: bool, // if true it's white to move
    pub white_king_castle: bool, // if a castle is avalable for the white kingside rook
    pub white_queen_castle: bool, // if a castle is avalable for the white queenside rook
    pub black_king_castle: bool, // if a castle is avalable for the black kingside rook
    pub black_queen_castle: bool, // if a castle is avalable for the black queenside rook
    pub en_passent: Option<Pos>,
    pub half_moves: usize, // the amount of half moves that has been made since the last capture or pawn advance
    pub full_moves: usize, // the amount of moves (a move is when each player has moved a piece) made in the whole game
}

impl ChessBoard {
    pub fn from_fen(fen: String) -> Self {
        let splitted_fen = fen.split_whitespace()
            .collect::<Vec<&str>>();

        let mut board: [Piece; 64] = [Piece::Empty; 64];
        for (y, row) in splitted_fen[0].split('/').enumerate() {
            let mut x: usize = 0;
            for char in row.chars() {
                if char.is_numeric() {
                    x += char.to_digit(10).unwrap() as usize;
                } else if char.is_alphabetic() {
                    board[y * 8 + x] = Piece::from_char(char);
                    x += 1;
                }
            }
        }

        let turn: bool = splitted_fen[1] == "w";

        let castle: [bool; 4] = [
            splitted_fen[2].contains('K'),
            splitted_fen[2].contains('Q'),
            splitted_fen[2].contains('k'),
            splitted_fen[2].contains('q'),
        ];

        let en_passent: Option<Pos> = Pos::from_chess_notation(splitted_fen[3].to_string()); 

        let half_moves: usize = splitted_fen[4].parse().unwrap();
        let full_moves: usize = splitted_fen[5].parse().unwrap();

        Self {
            board,
            turn,
            white_king_castle: castle[0],
            white_queen_castle: castle[1],
            black_king_castle: castle[2],
            black_queen_castle: castle[3],
            en_passent,
            half_moves,
            full_moves,
        }
    }

    pub fn available_moves() -> Vec<ChessMove> {
        
    }
}
