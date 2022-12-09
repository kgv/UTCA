use crate::{taxonomy::Taxonomy, utils::UiExt, Input};
use egui::{
    CollapsingHeader, Direction, Layout, Response, RichText, ScrollArea, TextStyle, Ui, Widget,
    Window,
};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};

/// List
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListWidget {
    pub(crate) data: Option<Input>,
    pub(crate) edit: bool,
    pub(crate) expand: Option<bool>,
    pub(crate) info: bool,
}

impl Widget for &mut ListWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let response = ui.vertical_centered(|ui| ui.heading("Input")).response;
        if let Some(data) = &mut self.data {
            let size = TextStyle::Body.resolve(ui.style()).size * 1.5;
            let species = data.species();
            let fatty_acids = data.fatty_acids();
            ScrollArea::vertical()
                .auto_shrink([false; 2])
                .show(ui, |ui| {
                    for specie in &species {
                        CollapsingHeader::new(RichText::from(specie.name()).heading())
                            .open(self.expand)
                            .show(ui, |ui| {
                                TableBuilder::new(ui)
                                    .vscroll(false)
                                    .striped(true)
                                    .cell_layout(Layout::centered_and_justified(
                                        Direction::LeftToRight,
                                    ))
                                    .column(Column::auto().resizable(true))
                                    .columns(Column::remainder(), 3)
                                    .column(Column::auto().resizable(true))
                                    .header(size, |mut row| {
                                        row.col(|_ui| {});
                                        row.col(|ui| {
                                            ui.label("sn 1, 3");
                                        });
                                        row.col(|ui| {
                                            ui.label("sn 2");
                                        });
                                        row.col(|ui| {
                                            ui.label("sn 1, 2, 3");
                                        });
                                        row.col(|ui| {
                                            if ui.button("+").clicked() {
                                                Window::new("").show(ui.ctx(), |ui| {
                                                    ui.label("text");
                                                });
                                                // .context_menu(|ui| {
                                                //     if ui.button("Expand all").clicked() {
                                                //         self.expand = Some(true);
                                                //         ui.close_menu();
                                                //     }
                                                //     if ui.button("Collapse all").clicked() {
                                                //         self.expand = Some(false);
                                                //         ui.close_menu();
                                                //     }
                                                // })
                                                // input[taxonomy].insert(default(), default());
                                            }
                                        });
                                    })
                                    .body(|mut body| {
                                        let mut percent = [0.0; 3];
                                        for fatty_acid in &fatty_acids {
                                            body.row(size, |mut row| {
                                                row.col(|ui| {
                                                    if self.edit {
                                                        ui.text_edit_singleline(
                                                            &mut fatty_acid.to_string(),
                                                        );
                                                    } else {
                                                        ui.label(fatty_acid.to_string());
                                                    }
                                                });
                                                if let Some(value) =
                                                    data[specie].get_mut(fatty_acid)
                                                {
                                                    percent[0] += value[0];
                                                    percent[1] += value[1];
                                                    percent[2] += value[2];
                                                    for value in value {
                                                        row.col(|ui| {
                                                            if self.edit {
                                                                ui.drag_percent(value);
                                                            } else {
                                                                ui.label(format!("{value:.1}%"));
                                                            }
                                                        });
                                                    }
                                                    row.col(|ui| {
                                                        if self.edit && ui.button("-").clicked() {
                                                            data[specie].remove(fatty_acid);
                                                        }
                                                    });
                                                }
                                            });
                                        }
                                        body.row(size, |mut row| {
                                            row.col(|ui| {
                                                ui.label("∑");
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{:.1}%", percent[0]))
                                                    .on_hover_text(
                                                        "Fatty acids in sn 1,3 position",
                                                    );
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{:.1}%", percent[1]))
                                                    .on_hover_text("Fatty acids in sn 2 position");
                                            });
                                            row.col(|ui| {
                                                ui.label(format!("{:.1}%", percent[2]))
                                                    .on_hover_text(
                                                        "Fatty acids in sn 1, 2, 3 position",
                                                    );
                                            });
                                        });
                                    });
                            })
                            .header_response
                            .on_hover_text(specie.taxonomy("."));
                    }
                });
        }
        response
    }
}
