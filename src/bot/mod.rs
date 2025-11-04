mod uci;
mod search;

use {
    search::{
        SearchThread,
    },
    chess::{
        Board, ChessMove, Error as ChessError, Piece, Square,
    },
    std::{
        error::Error,
        fmt::{
            Debug,
            Display,
        },
        io::{
            stdin,
            stdout,
            BufRead,
            Error as IoError,
            Write,
        },
        str::{
            FromStr,
            SplitWhitespace,
        },
        time::Duration,
    },
};

pub use uci::UCI;

#[derive(Debug, Clone, Default)]
pub struct Engine {
    search_thread: SearchThread,
    current_board: Option<Board>,
    moves: Vec<ChessMove>,
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
    search_moves: Vec<ChessMove>,
    ponder: bool,
    white_time: Option<Duration>,
    black_time: Option<Duration>,
    white_increment_time: Duration,
    black_increment_time: Duration,
    moves_to_go: usize,
    depth: usize,
    nodes: usize,
    mate: usize,
    move_time: MoveTime,

    // Extra
    board: Board,
    // moves: Vec<ChessMove>,
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
            // debug [ on | off ]
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
                Some("isready") => self.isready(),
                Some("ucinewgame") => self.ucinewgame(),
                Some("position") => self.position(&mut parts),
                Some("go") => self.go(&mut parts),
                Some("quit") => break, // TODO: when making the quit method from the UCI trait. You should stop the thread by setting the stop_flag and then afterwards drop the sender value to stop the listen loop
                _ => Ok(()),
            };

            if result.is_err() {
                writeln!(stdout, "{}", result.err().unwrap())?;
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

    fn isready(&self) -> Result<(), EngineError> {
        let mut stdout = stdout();

        // TODO: make sure the engine isn't calculating or anything

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
                    return Err(EngineError::InvalidCommand("position fen".to_string()))
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
                        return Err(EngineError::InvalidCommand("position ... moves".to_string()))
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
        let mut options = GoOptions::default();

        while let Some(subcommand) = arguments.next() {
            match subcommand {
                "searchmoves" => {
                    while let Some(move_notation) = arguments.next() {
                        if move_notation.len() < 4 || move_notation.len() > 5 {
                            return Err(EngineError::InvalidCommand("go searchmoves".to_string()))
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

        options.board = self.current_board.ok_or(EngineError::InvalidCommand("The position has never been initialized".to_string()))?;

        self.search_thread.sender.send(options).unwrap();

        Ok(())
    }
}
