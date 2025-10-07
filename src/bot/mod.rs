use {
    chess::{
        Board,
        ChessMove,
        Square,
        Piece,
    },
    std::{
        io::{
            stdin,
            stdout,
            BufRead,
            Result,
            Write,
        },
        fmt::Debug,
        str::FromStr,
        time::Duration,
    },
};

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default)]
enum MoveTime {
    #[default]
    NotSpecified,
    Finite(Duration),
    Infinite,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash, Default)]
struct CalculatingOptions {
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
}

#[derive(Debug, Clone)]
struct Game {
    current_board: Board,
    moves: Vec<ChessMove>,
}

#[derive(Debug, Clone, Default)]
pub struct Engine {
    game: Option<Game>,
    // Potential cache and data for the engine
}

impl Engine {
    pub fn listen(&mut self) -> Result<()> {
        let stdin = stdin();
        let mut stdout = stdout();

        for line in stdin.lock().lines() {
            let line = line.unwrap();
            let mut parts = line.split_whitespace();

            // TODO: Optimize the position command, so it doesnt rebuild the whole board if it's just a few moves behind
            // TODO: Split this up into multiple functions
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

            match parts.next() {
                Some("uci") => {
                    writeln!(stdout, "id name ChessEngine")?;
                    writeln!(stdout, "id author Lukas Elias Lund Majland")?;
                    writeln!(stdout, "uciok")?;
                    stdout.flush()?;
                },
                Some("isready") => {
                    writeln!(stdout, "readyok")?;
                    stdout.flush()?;
                },
                Some("ucinewgame") => self.reset(),
                Some("position") => {
                    // TODO: Maybe write this as a match statement?


                    let position_op = parts.next();

                    if position_op.is_some() {
                        if position_op == Some("startpos") {
                            self.game = Some(Game {
                                current_board: Board::default(),
                                moves: Vec::new(),
                            });
                        } else if position_op == Some("fen") {
                            let mut fen = String::new();

                            for _ in 0..6 {
                                fen.push_str(parts.next().expect("This is an invalid fen string. The program will now panic. TODO: Don't panic here 👍"));
                                fen.push(' ');
                            }

                            self.game = Some(Game {
                                current_board: Board::from_str(fen.as_str()).unwrap(),
                                moves: Vec::new(),
                            })
                        }

                        if let Some(game) = &mut self.game {
                            if parts.next() == Some("moves") {
                                while let Some(move_notation) = parts.next() {
                                    let src_square = Square::from_str(&move_notation[0..2]).expect("You gave the wrong input here dummy");
                                    let dest_square = Square::from_str(&move_notation[2..4]).expect("Same on this line dummy");
                                    let promotion = match move_notation.chars().nth(4) {
                                        Some('q') => Some(Piece::Queen),
                                        Some('n') => Some(Piece::Knight),
                                        Some('r') => Some(Piece::Rook),
                                        Some('b') => Some(Piece::Bishop),
                                        Some(_) => None,
                                        None => None,
                                    };
                                    
                                    let chess_move = ChessMove::new(src_square, dest_square, promotion);

                                    game.current_board = game.current_board.make_move_new(chess_move);

                                    game.moves.push(chess_move);
                                }
                            }
                        }
                    }
                },
                Some("go") => {
                    // TODO: understand the sub commands for the go command and make a CalculatingOptions struct and pass it to the go function
                    // If both movetime and infinite for some reason showed up, the last one will be the one the engine thinks is valid

                    let mut options = CalculatingOptions::default();

                    while let Some(subcommand) = parts.next() {
                        match subcommand {
                            "searchmoves" => {},
                            "ponder" => {
                                options.ponder = true;
                            },
                            "wtime" => {
                                let millisec: u64 = parts.next().expect("The parameters is incorrect for wtime, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic. Thanks for now. TODO: Don't panic");
                                options.white_time = Some(Duration::from_millis(millisec));
                            },
                            "btime" => {
                                let millisec: u64 = parts.next().expect("The parameters is incorrect for btime, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic. Thanks for now. TODO: Don't panic");
                                options.black_time = Some(Duration::from_millis(millisec));
                            },
                            "winc" => {
                                let millisec: u64 = parts.next().expect("The parameters is incorrect for winc, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic Thanks for now. TODO: Don't panic");
                                options.white_increment_time = Duration::from_millis(millisec);
                            },
                            "binc" => {
                                let millisec: u64 = parts.next().expect("The parameters is incorrect for binc, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic Thanks for now. TODO: Don't panic");
                                options.black_increment_time = Duration::from_millis(millisec);
                            },
                            "movestogo" => {
                                let moves: usize = parts.next().expect("The parameters is incorrect for movestogo, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic. Thanks for now. TODO: Don't panic");
                                options.moves_to_go = moves;
                            },
                            "depth" => {
                                let depth: usize = parts.next().expect("The parameters is incorrect for depth, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic. Thanks for now. TODO: Don't panic");
                                options.depth = depth;
                            },
                            "nodes" => {
                                let nodes: usize = parts.next().expect("The parameters is incorrect for nodes, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic. Thanks for now. TODO: Don't panic");
                                options.nodes = nodes;
                            },
                            "mate" => {
                                let mate: usize = parts.next().expect("The parameters is incorrect for mate, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic. Thanks for now. TODO: Don't panic");
                                options.mate = mate;
                            },
                            "movetime" => {
                                let millisec: u64 = parts.next().expect("The parameters is incorrect for movetime, will panic. TODO: Don't panic").parse().expect("The parameter is not a number that works. Atleast it threw an error parsing it, so I'll panic. Thanks for now. TODO: Don't panic");
                                options.move_time = MoveTime::Finite(Duration::from_millis(millisec));
                            },
                            "infinite" => {
                                options.move_time = MoveTime::Infinite;
                            },
                            _ => { }
                        }
                    }

                    self.go(options);
                }
                Some("quit") => break,
                _ => { },
            }

            // debug
            if let Some(game) = &self.game {
                writeln!(stdout, "info string board {} moves {:?}", game.current_board, game.moves)?;
                stdout.flush()?;
            }
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.game = None;
    }

    fn go(&self, options: CalculatingOptions) {
        // TODO: calculate move

        println!("go: {:?}", options);
    }
}
