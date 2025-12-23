mod uci;
mod pst;

use {
    pst::*,
    chess::{
        Board, CastleRights, ChessMove, Error as ChessError, MoveGen, Piece, Square, ALL_PIECES,
    },
    std::{
        error::Error,
        fmt::{
            Debug,
            Display,
        }, io::{
            stdin,
            stdout,
            BufRead,
            Error as IoError,
            Write,
        }, str::{
            FromStr,
            SplitWhitespace,
        }, time::{
            Duration,
            Instant,
        }
    },
};

pub use uci::UCI;

#[derive(Debug, Clone, Default)]
pub struct Engine {
    current_board: Option<Board>,
    moves: Vec<ChessMove>,
    debug: bool,
    // Potential cache and data for the engine
}

#[derive(Debug)]
pub enum EngineError {
    InvalidCommand(String),
    Chess(ChessError),
    Io(IoError),
}

impl Display for EngineError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCommand(string) => write!(f, "Invalid UCI Command: {}", string),
            Self::Chess(error) => write!(f, "An error with the chess rust crate has occured: {}", error),
            Self::Io(error) => write!(f, "An I/O error has occured: {}", error),
        }
    }
}

impl Error for EngineError {}

impl From<ChessError> for EngineError {
    fn from(value: ChessError) -> Self {
        Self::Chess(value)
    }
}

impl From<IoError> for EngineError {
    fn from(value: IoError) -> Self {
        Self::Io(value)
    }
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
pub struct GoOptions {
    pub search_moves: Vec<ChessMove>,
    pub ponder: bool,
    pub white_time: Option<Duration>,
    pub black_time: Option<Duration>,
    pub white_increment_time: Duration,
    pub black_increment_time: Duration,
    pub moves_to_go: usize,
    pub depth: usize,
    pub nodes: usize,
    pub mate: usize,
    pub move_time: MoveTime,
}

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
pub enum MoveTime {
    #[default]
    NotSpecified,
    Finite(Duration),
    Infinite,
}

impl UCI for Engine {
    fn listen(&mut self) -> Result<(), EngineError> {
        let stdin = stdin();
        let mut stdout = stdout();

        for line in stdin.lock().lines() {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();

            // TODO: Optimize the position command, so it doesnt rebuild the whole board if it's just a few moves behind
            // TODO: Support every command from the gui:
            //
            // setoption name  [value ]
            // 
            // register
            //   later
            //   name 
            //   code 
            // 
            // go
            //   searchmoves  .... 
            //   ponder
            //   wtime x
            //   btime x
            //   winc x
            //   binc x
            //   movestogo x
            //   depth x
            //   nodes x
            //   mate x
            //   movetime x
            //   infinite
            //   
            // stop
            // 
            // ponderhit
            //

            let result = match parts.next() {
                Some("uci") => self.uci(),
                Some("debug") => self.debug(&mut parts),
                Some("isready") => self.isready(),
                Some("ucinewgame") => self.ucinewgame(),
                Some("position") => self.position(&mut parts),
                Some("go") => self.go(&mut parts),
                Some("quit") => break,
                _ => Ok(()),
            };

            if result.is_err() && self.debug {
                writeln!(stdout, "info string {}", result.err().unwrap())?;
                stdout.flush()?;
            }
        }

        Ok(())
    }

    fn uci(&self) -> Result<(), EngineError> {
        let mut stdout = stdout();

        writeln!(stdout, "id name ChessEngine")?;
        writeln!(stdout, "id author Lukas Elias Lund Majland")?;
        writeln!(stdout, "uciok")?;
        stdout.flush()?;

        Ok(())
    }

    fn debug(&mut self, arguments: &mut SplitWhitespace) -> Result<(), EngineError> {
        self.debug = match arguments.next() {
            Some("on") => true,
            Some(_) => false,
            None => return Err(EngineError::InvalidCommand("Missed argument to debug".to_string())),
        };

        Ok(())
    }

    fn isready(&self) -> Result<(), EngineError> {
        let mut stdout = stdout();

        writeln!(stdout, "readyok")?;
        stdout.flush()?;

        Ok(())
    }

    fn ucinewgame(&mut self) -> Result<(), EngineError> {
        self.current_board = None;
        self.moves.clear();

        Ok(())
    }

    fn position(&mut self, arguments: &mut SplitWhitespace) -> Result<(), EngineError> {
        let mut board: Board;
        match arguments.next() {
            Some("startpos") => board = Board::default(),
            Some("fen") => {
                let fen: Vec<&str> = arguments.take(6).collect();

                if fen.len() != 6 {
                    return Err(EngineError::InvalidCommand("position fen".to_string()));
                }

                board = Board::from_str(fen.join(" ").as_str())?;
            },
            _ => return Err(EngineError::InvalidCommand("position".to_string())),
        }

        let mut moves: Vec<ChessMove> = Vec::new();

        match arguments.next() {
            Some("moves") => {
                while let Some(move_notation) = arguments.next() {
                    if move_notation.len() < 4 || move_notation.len() > 5 {
                        return Err(EngineError::InvalidCommand("position ... moves".to_string()));
                    }

                    let src_square = Square::from_str(&move_notation[0..2])?;
                    let dest_square = Square::from_str(&move_notation[2..4])?;
                    let promotion = match move_notation.chars().nth(4) {
                        Some('q') => Some(Piece::Queen),
                        Some('n') => Some(Piece::Knight),
                        Some('r') => Some(Piece::Rook),
                        Some('b') => Some(Piece::Bishop),
                        Some(_) => None,
                        None => None,
                    };

                    let chess_move = ChessMove::new(src_square, dest_square, promotion);
                    board = board.make_move_new(chess_move);
                    moves.push(chess_move);
                }
            }
            _ => ()
        }

        self.current_board = Some(board);
        self.moves = moves;

        Ok(())
    }

    fn go(&self, arguments: &mut SplitWhitespace) -> Result<(), EngineError> {
        let mut stdout = stdout();

        let mut options = GoOptions::default();

        while let Some(subcommand) = arguments.next() {
            match subcommand {
                "searchmoves" => {
                    while let Some(move_notation) = arguments.next() {
                        if move_notation.len() < 4 || move_notation.len() > 5 {
                            return Err(EngineError::InvalidCommand("go searchmoves".to_string()));
                        }

                        let src_square = Square::from_str(&move_notation[0..2])?;
                        let dest_square = Square::from_str(&move_notation[2..4])?;
                        let promotion = match move_notation.chars().nth(4) {
                            Some('q') => Some(Piece::Queen),
                            Some('n') => Some(Piece::Knight),
                            Some('r') => Some(Piece::Rook),
                            Some('b') => Some(Piece::Bishop),
                            Some(_) => None,
                            None => None,
                        };

                        let chess_move = ChessMove::new(src_square, dest_square, promotion);
                        options.search_moves.push(chess_move);
                    }
                },
                "ponder" => options.ponder = true,
                "wtime" => {
                    let millisec: u64 = arguments.next().ok_or(EngineError::InvalidCommand("go wtime".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go wtime".to_string()))?;
                    options.white_time = Some(Duration::from_millis(millisec));
                },
                "btime" => {
                    let millisec: u64 = arguments.next().ok_or(EngineError::InvalidCommand("go btime".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go btime".to_string()))?;
                    options.black_time = Some(Duration::from_millis(millisec));
                },
                "winc" => {
                    let millisec: u64 = arguments.next().ok_or(EngineError::InvalidCommand("go winc".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go winc".to_string()))?;
                    options.white_increment_time = Duration::from_millis(millisec);
                },
                "binc" => {
                    let millisec: u64 = arguments.next().ok_or(EngineError::InvalidCommand("go binc".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go binc".to_string()))?;
                    options.black_increment_time = Duration::from_millis(millisec);
                },
                "movestogo" => {
                    let moves: usize = arguments.next().ok_or(EngineError::InvalidCommand("go movestogo".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go movestogo".to_string()))?;
                    options.moves_to_go = moves;
                },
                "depth" => {
                    let depth: usize = arguments.next().ok_or(EngineError::InvalidCommand("go depth".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go depth".to_string()))?;
                    options.depth = depth;
                },
                "nodes" => {
                    let nodes: usize = arguments.next().ok_or(EngineError::InvalidCommand("go nodes".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go nodes".to_string()))?;
                    options.nodes = nodes;
                },
                "mate" => {
                    let mate: usize = arguments.next().ok_or(EngineError::InvalidCommand("go mate".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go mate".to_string()))?;
                    options.mate = mate;
                },
                "movetime" => {
                    let millisec: u64 = arguments.next().ok_or(EngineError::InvalidCommand("go movetime".to_string()))?.parse().map_err(|_| EngineError::InvalidCommand("go movetime".to_string()))?;
                    options.move_time = MoveTime::Finite(Duration::from_millis(millisec));
                },
                "infinite" => {
                    options.move_time = MoveTime::Infinite;
                },
                _ => (),
            }
        }

        let calculated_move = self.search_moves(options)?;

        // Make a move

        writeln!(stdout, "bestmove {}", calculated_move)?;
        stdout.flush()?;

        Ok(())
    }
}

// TODO: Optimize and test how fast it is
// TODO: check the go_options etc...

impl Engine {
    fn search_moves(&self, go_options: GoOptions) -> Result<ChessMove, EngineError> {
        let mut stdout = stdout();
        let now = Instant::now();

        let board = &self.current_board.ok_or(EngineError::InvalidCommand("No position given".to_string()))?;

        let move_gen = MoveGen::new_legal(board);
        let mut moves: Vec<(isize, ChessMove)> = Vec::new();
        let mut highest_score_index: usize = 0;

        for child in move_gen {
            let child_board = board.make_move_new(child);
            let score = minimax(&child_board, false, 3);

            moves.push((score, child));

            if moves[highest_score_index].0 <= score {
                highest_score_index = moves.len() - 1;
            }
        }

        let elapsed = now.elapsed();
        writeln!(stdout, "info string Elapsed time for the search: {:.2?}", elapsed)?;
        stdout.flush()?;

        Ok(moves[highest_score_index].1)
    }
}

fn minimax(board: &Board, maximizing: bool, depth: usize) -> isize {
    if depth == 0 {
        return evaluate(board, maximizing);
    }

    let move_gen = MoveGen::new_legal(board);

    if maximizing {
        let mut max = f32::NEG_INFINITY as isize;

        for chess_move in move_gen {
            let new_board = board.make_move_new(chess_move);
            let score = minimax(&new_board, false, depth - 1);

            max = max.max(score);
        }
        
        return max;
    } else {
        let mut min = f32::INFINITY as isize;

        for chess_move in move_gen {
            let new_board = board.make_move_new(chess_move);
            let score = minimax(&new_board, true, depth - 1);

            min = min.min(score);
        }
        
        return min;
    }
}

fn evaluate(board: &Board, maximizing: bool) -> isize {
    let maximizing_player = if maximizing { board.side_to_move() } else { !board.side_to_move() };

    let mut score = 0;
    let max_castle_rights = board.castle_rights(maximizing_player);
    let min_castle_rights = board.castle_rights(!maximizing_player);

    score += castle_rights_to_score(max_castle_rights);
    score -= castle_rights_to_score(min_castle_rights);

    let max_pieces = board.color_combined(maximizing_player);
    let min_pieces = board.color_combined(!maximizing_player);

    for (piece, pst) in std::array::from_fn::<(Piece, PieceSquareTable), 6, _>(|i| (ALL_PIECES[i], ALL_PSTS[i])) {
        let max_bit_board = board.pieces(piece) & max_pieces;

        score += max_bit_board.popcnt() as isize * piece_to_score(piece);

        score += pst.to_score(&max_bit_board);

        let min_bit_board = board.pieces(piece) & min_pieces;

        score -= min_bit_board.popcnt() as isize * piece_to_score(piece);

        score -= pst.to_score(&min_bit_board);
    }

    score
}

fn castle_rights_to_score(rights: CastleRights) -> isize {
    match rights {
        CastleRights::NoRights => 0,
        CastleRights::KingSide | CastleRights::QueenSide => 300,
        CastleRights::Both => 600,
    }
}

fn piece_to_score(piece: Piece) -> isize {
    match piece {
        Piece::Pawn => 100,
        Piece::Knight => 320,
        Piece::Bishop => 330,
        Piece::Rook => 500,
        Piece::Queen => 900,
        Piece::King => 20000,
    }
}
