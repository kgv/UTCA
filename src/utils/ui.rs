use eframe::emath::Numeric;
use egui::{DragValue, Response, Sense, Ui, Vec2, Widget};

pub(crate) trait UiExt {
    fn default_response(&mut self) -> Response;

    fn drag_percent<T: Numeric>(&mut self, value: &mut T) -> Response;
}

impl UiExt for Ui {
    fn default_response(&mut self) -> Response {
        self.allocate_response(Vec2::default(), Sense::hover())
    }

    fn drag_percent<T: Numeric>(&mut self, value: &mut T) -> Response {
        DragValue::new(value)
            .clamp_range(0..=100)
            .speed(0.1)
            .suffix('%')
            .ui(self)
    }
}
