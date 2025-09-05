use {
    chess::{
        Board,
        ChessMove,
        MoveGen,
    },
    std::time::Duration,
};

#[derive(Debug, Clone)]
pub struct Game<W: Player, B: Player> {
    current_board: Board,
    moves: Vec<ChessMove>,
    player_white: W,
    player_black: B,
    time_left: Option<Duration>,
    start_time: Option<Duration>,
}

impl<W: Player, B: Player> Game<W, B> {
    pub fn new(player_white: W, player_black: B) -> Self {
        Self {
            current_board: Board::default(),
            moves: Vec::new(),
            player_white,
            player_black,
            time_left: None,
            start_time: None,
        }
    }
}

pub trait Player {
    fn make_move(board: &Board) -> ChessMove;
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Bot;

impl Player for Bot {
    fn make_move(board: &Board) -> ChessMove {
        MoveGen::new_legal(board).next().unwrap()
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct Human;

impl Player for Human {
    fn make_move(board: &Board) -> ChessMove {
        MoveGen::new_legal(board).next().unwrap()
    }
}
