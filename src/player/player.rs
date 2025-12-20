pub struct Player {
    pub name: String,
}

impl Player {
    pub fn new() -> Self {
        Self {
            name: "Initiate".to_string(),
        }
    }
}
