use imgui::Ui;

pub fn tooltip<F>(ui: &Ui, item: F) 
where
    F: FnOnce() -> ()
{
    if ui.is_item_hovered() {
        ui.tooltip(item);
    }
}


fn popup<F>(ui: &Ui, button_label: impl AsRef<str>, ack_label: impl AsRef<str>, id: u32, item: F) 
where
    F: FnOnce() -> ()
{
    if ui.button(button_label) {
        ui.open_popup(format!("{id}a"));
    }
    if let Some(_token) = ui.begin_modal_popup(format!("{id}b")) {
        item();
        if ui.button(ack_label) {
            ui.close_current_popup();
        }
    };
}

fn popup_no_button<F>(ui: &Ui, ack_label: impl AsRef<str>, id: u32, item: F) 
where
    F: FnOnce() -> ()
{
    let mut t = true;
    if t {
        ui.open_popup(format!("{id}a"));
        t = false;
    }
    if let Some(_token) = ui.begin_modal_popup(format!("{id}b")) {
        item();
        if ui.button(ack_label) {
            ui.close_current_popup();
        }
    };
}