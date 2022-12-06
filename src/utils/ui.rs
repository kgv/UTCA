use egui::{Response, Sense, Ui, Vec2};

pub(crate) trait UiExt {
    fn default_response(&mut self) -> Response;
}

impl UiExt for Ui {
    fn default_response(&mut self) -> Response {
        self.allocate_response(Vec2::default(), Sense::hover())
    }
}
