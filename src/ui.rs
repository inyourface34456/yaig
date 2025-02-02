use imgui::{Ui, Condition};
use crate::{componets::tooltip, Game, Tab};

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
    // if ui.button("Show modal") {
    //     ui.open_popup("modal");
    // }
    // if let Some(_token) = ui.begin_modal_popup("modal") {
    //     ui.text("Content of my modal");
    //     if ui.button("OK") {
    //         ui.close_current_popup();
    //     }
    // };

    ui.text(format!("{}", game.count));

    if ui.button("incrmentm count") {
        game.count += 1;
    } 
}

fn setting(ui: &Ui, game: &mut Game) {
    // ui.checkbox("Fullscreen", &mut game.config.fullscreen);
    ui.text("Fullscreen Mode:");
    ui.radio_button("Windowed", &mut game.config.fullscreen, crate::settings::Screen::Windowed);
    tooltip(&ui, || {ui.text("Standered Window");});

    ui.radio_button("Borderless", &mut game.config.fullscreen, crate::settings::Screen::Borderless);
    tooltip(&ui, || {ui.text("Sandered Ftullscreen");});
    
    ui.radio_button("Exclusive", &mut game.config.fullscreen, crate::settings::Screen::Fullscreen);
    tooltip(&ui, || {ui.text("Fullscreen, but this is the ONLY thing allowed on this moniter.");});
}