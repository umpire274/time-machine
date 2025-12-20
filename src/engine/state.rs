use crate::player::player::Player;

pub struct GameState {
    pub running: bool,
    pub player: Player,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            running: true,
            player: Player::new(),
        }
    }
}
