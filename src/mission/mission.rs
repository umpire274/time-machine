use crate::world::epoch::Epoch;

pub struct Mission {
    pub epoch: Epoch,
    pub briefing: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MissionKind {
    AncientEgypt,
    AncientRome,
    MiddleAges,
    Renaissance,
    WorldWarII,
}

impl MissionKind {
    pub fn all() -> &'static [MissionKind] {
        &[
            MissionKind::AncientEgypt,
            MissionKind::AncientRome,
            MissionKind::MiddleAges,
            MissionKind::Renaissance,
            MissionKind::WorldWarII,
        ]
    }

    pub fn epoch(self) -> Epoch {
        match self {
            MissionKind::AncientEgypt => Epoch::AncientEgypt,
            MissionKind::AncientRome => Epoch::AncientRome,
            MissionKind::MiddleAges => Epoch::MiddleAges,
            MissionKind::Renaissance => Epoch::Renaissance,
            MissionKind::WorldWarII => Epoch::WorldWarII,
        }
    }

    pub fn display_name(self) -> &'static str {
        match self {
            MissionKind::AncientEgypt => "Ancient Egypt",
            MissionKind::AncientRome => "Ancient Rome",
            MissionKind::MiddleAges => "Middle Ages",
            MissionKind::Renaissance => "Renaissance",
            MissionKind::WorldWarII => "Europe – World War II",
        }
    }

    pub fn is_available(self) -> bool {
        matches!(self, MissionKind::AncientEgypt)
    }
}
