use crate::engine::game::Game;

mod engine;
mod mission;
mod player;
mod ui;
mod world;

fn main() {
    let mut game = Game::new();
    game.run();
}
