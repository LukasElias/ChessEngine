mod bot;

use bot::{
    Game,
    Bot,
};

fn main() {
    let white = Bot::default();
    let black = Bot::default();

    let game = Game::new(white, black);
}
