use imgui::{Ui, Condition};
use crate::{Game, Tab};

pub fn main_ui(ui: &mut Ui, mut game_state: &mut Game) {
    ui.window("Hello world")
        .size([game_state.width, game_state.height], Condition::Always)
        .resizable(false)
        .collapsible(false)
        .movable(false)
        .title_bar(false)
        .position([0., 0.], Condition::Always)
        .build(|| {
            if ui.button("Game") {
                game_state.active = Tab::Game;
            }
            ui.same_line();
            if ui.button("Settings") {
                game_state.active = Tab::Settings
            }
            ui.separator();

            match game_state.active {
                Tab::Game => main_game(&ui, &mut game_state),
                Tab::Settings => setting(&ui, &mut game_state)
            }
        });
}

fn main_game(ui: &Ui, game: &mut Game) {
    ui.text(format!("{}", game.count));

    if ui.button("incrmentm count") {
        game.count += 1;
    }
    ui.text("tab1");
}

fn setting(ui: &Ui, game: &mut Game) {
    ui.checkbox("Fullscreen", &mut game.config.fullscreen);
}