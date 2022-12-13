use egui::{ScrollArea, Ui};
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Text {
    pub text: String,
}

impl Text {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.centered_and_justified(|ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                ui.code_editor(&mut self.text);
            });
        });
    }
}
