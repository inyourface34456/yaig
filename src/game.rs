use crate::Settings;

pub enum Tab {
    Game,
    Settings,
}

impl Default for Tab {
    fn default() -> Self {
        Self::Game
    }
}

#[derive(Default)]
pub struct Game {
    pub count: u128,
    pub width: f32,
    pub height: f32,
    pub active: Tab,
    pub config: Settings,
}