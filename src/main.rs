mod backend;
mod bot;

use {
    std::sync::Arc,
    tokio::sync::Mutex,
    bot::{
        Game,
        Human,
        Bot,
        Player,
    },
};

#[tokio::main]
async fn main() {
    let white = Human::default();
    let black = Bot::default();

    let game = Arc::new(Mutex::new(Game::new(white, black)));

    tokio::spawn(async move { backend::run_server(game.clone()).await });

    loop {
    }
}
