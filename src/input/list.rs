use crate::{utils::UiExt, Input};
use egui::{CollapsingHeader, Direction, Layout, RichText, ScrollArea, TextStyle, Ui, Window};
use egui_extras::{Column, TableBuilder};
use serde::{Deserialize, Serialize};

/// List
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct List {
    pub input: Input,
    pub edit: bool,
    pub expand: Option<bool>,
}

impl List {
    pub fn ui(&mut self, ui: &mut Ui) {
        ui.vertical_centered(|ui| ui.heading("Input"));
        ui.separator();
        let size = TextStyle::Body.resolve(ui.style()).size * 1.5;
        let open = self.expand.take();
        let species = self.input.species();
        let fatty_acids = self.input.fatty_acids();
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for specie in &species {
                    CollapsingHeader::new(RichText::from(specie.to_string()).heading())
                        .open(open)
                        .show(ui, |ui| {
                            TableBuilder::new(ui)
                                .striped(true)
                                .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                                .columns(Column::auto().resizable(true), 4)
                                .column(Column::exact(size).resizable(true))
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
                                    row.col(|_ui| {});
                                    // if self.edit {
                                    //     row.col(|ui| {
                                    //         if ui.button("+").clicked() {
                                    //             Window::new("").show(ui.ctx(), |ui| {
                                    //                 ui.label("text");
                                    //             });
                                    //             // .context_menu(|ui| {
                                    //             //     if ui.button("Expand all").clicked() {
                                    //             //         self.expand = Some(true);
                                    //             //         ui.close_menu();
                                    //             //     }
                                    //             //     if ui.button("Collapse all").clicked() {
                                    //             //         self.expand = Some(false);
                                    //             //         ui.close_menu();
                                    //             //     }
                                    //             // })
                                    //             // input[taxonomy].insert(default(), default());
                                    //         }
                                    //     });
                                    // }
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
                                                self.input[specie].get_mut(fatty_acid)
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
                                                        self.input[specie].remove(fatty_acid);
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
                                                .on_hover_text("Fatty acids in sn 1,3 position");
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{:.1}%", percent[1]))
                                                .on_hover_text("Fatty acids in sn 2 position");
                                        });
                                        row.col(|ui| {
                                            ui.label(format!("{:.1}%", percent[2])).on_hover_text(
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
}
