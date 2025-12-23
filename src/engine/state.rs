use crate::mission::mission::MissionKind;
use crate::player::player::Player;

pub struct GameState {
    pub running: bool,
    pub player: Player,
    pub location: Location,
    pub started: bool,
    pub selected_mission: Option<MissionKind>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Location {
    Splash,
    BaseTmo,
    InMission,
}

impl GameState {
    pub fn new() -> Self {
        Self {
            running: true,
            player: Player::new(),
            location: Location::Splash,
            started: false,
            selected_mission: None,
        }
    }
}
