#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Screen {
    Windowed,
    Fullscreen,
    Borderless
}

impl Default for Screen {
   fn default() -> Self {
       Self::Windowed
   } 
}

#[derive(Default)]
pub struct Settings {
    pub fullscreen: Screen,
}