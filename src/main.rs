mod backend;
mod bot;

use {
    bot::{
        Game,
        Human,
        Bot,
    },
};

#[tokio::main]
async fn main() {
    let white = Human::default();
    let black = Bot::default();

    let game = Game::new(white, black);

    tokio::spawn(async move { backend::run_server().await });

    loop {
    }
}
