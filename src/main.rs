// mod backend;
mod bot;

use {
    std::io::{
        stdin,
        stdout,
        Write,
        BufRead,
        Result,
    },
    bot::{
        Game,
        Human,
        Bot,
        Player,
    },
};

fn main() -> Result<()> {
    let stdin = stdin();
    let mut stdout = stdout();
    
    for line in stdin.lock().lines() {
        let cmd = line.unwrap(); // each line is a String without trailing newline

        match cmd.as_str() {
            "uci" => {
                writeln!(stdout, "id name ChessEngine")?;
                writeln!(stdout, "id author Lukas Elias Lund Majland")?;
                writeln!(stdout, "uciok")?;
                stdout.flush()?;
            }
            "isready" => {
            }
            "quit" => break,
            _ => { }
        }

    }

    Ok(())


    // let white = Human::default();
    // let black = Bot::default();
    //
    // let game = Game::new(white, black);
}
