use crate::engine::state::GameState;
use crate::engine::looping::game_loop;

pub struct Game {
    state: GameState,
}

impl Game {
    pub fn new() -> Self {
        Self {
            state: GameState::new(),
        }
    }

    pub fn run(&mut self) {
        game_loop(&mut self.state);
    }
}
