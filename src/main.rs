mod bot;

use {
    std::io::Result,
    bot::Engine,
};

fn main() -> Result<()> {
    let mut engine = Engine::default();

    engine.listen()
}
