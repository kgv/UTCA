use egui::Ui;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Text {
    pub text: String,
}

impl Text {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.code_editor(&mut self.text);
    }
}
