use {
    chess::{
        Board,
        ChessMove,
        Square,
        Piece,
    },
    std::{
        fmt::Debug,
        io::{
            stdin,
            stdout,
            BufRead,
            Result,
            Write,
        },
        str::FromStr,
    },
};

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

            // TODO: Split this up into multiple functions
            // TODO: Write info all the time to make sure how the engine is doing

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
}
