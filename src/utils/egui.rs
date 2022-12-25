use eframe::emath::Numeric;
use egui::{collapsing_header::CollapsingState, DragValue, Response, Sense, Ui, Vec2, Widget};

/// Extension methods for [`CollapsingState`]
pub(crate) trait CollapsingStateExt {
    fn open(self, open: Option<bool>) -> Self;
}

impl CollapsingStateExt for CollapsingState {
    fn open(mut self, open: Option<bool>) -> Self {
        if let Some(open) = open {
            self.set_open(open);
        }
        self
    }
}

/// Extension methods for [`Ui`]
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
