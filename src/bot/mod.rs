use {
    chess::{
        Board,
        ChessMove,
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

                            for i in 0..6 {
                                println!("{}", i);
                                fen.push_str(parts.next().expect("This is an invalid fen string. The program will now panic. TODO: Don't panic here 👍"));
                                fen.push(' ');
                            }

                            println!("fen: {}", fen);

                            self.game = Some(Game {
                                current_board: Board::from_str(fen.as_str()).unwrap(),
                                moves: Vec::new(),
                            })
                        }

                        // Apply the moves if any
                    }
                },
                Some("quit") => break,
                _ => { },
            }
        }

        Ok(())
    }

    fn reset(&mut self) {
        self.game = None;
    }
}
