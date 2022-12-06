use super::Input;
use crate::utils::UiExt;
use egui::{
    collapsing_header::CollapsingState, Align, CollapsingHeader, Direction, DragValue, Grid, Id,
    Layout, Response, Sense, TextStyle, Ui, Vec2, Widget,
};
use egui_extras::{Size, StripBuilder, TableBuilder};
use indexmap::{map::MutableKeys, IndexMap};
use serde::{Deserialize, Serialize};
use std::default::default;

/// List widget
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListWidget {
    pub(crate) input: Input,
    pub(crate) info: bool,
    pub(crate) expand: Option<bool>,
}

impl Default for ListWidget {
    fn default() -> Self {
        Self {
            input: default(),
            info: default(),
            expand: default(),
        }
    }
}

impl Widget for &mut ListWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        let unit = TextStyle::Body.resolve(ui.style()).size * 1.5;
        // CollapsingHeader::new(ui.ctx(), Id::new(taxonomy), false)
        //     .show_header(ui, |ui| {
        //         ui.label(taxonomy.to_string())
        //             .on_hover_text(format!("{taxonomy:#}"));
        //         ui.toggle_value(&mut self.info, "ℹ")
        //             .on_hover_text("Information");
        //     })
        // CollapsingHeader::new("Input")
        //     .default_open(true)
        //     .show(ui, |ui| {
        let response = ui
            .vertical_centered(|ui| {
                ui.heading("Input").context_menu(|ui| {
                    if ui.button("Expand all").clicked() {
                        self.expand = Some(true);
                        ui.close_menu();
                    }
                    if ui.button("Collapse all").clicked() {
                        self.expand = Some(false);
                        ui.close_menu();
                    }
                })
            })
            .response;
        for (taxonomy, value) in self.input.iter_mut() {
            CollapsingHeader::new(taxonomy.to_string())
                .open(self.expand)
                .show(ui, |ui| {
                    TableBuilder::new(ui)
                        .striped(true)
                        .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
                        .columns(Size::exact(unit * 3.0), 4)
                        .column(Size::exact(unit))
                        .header(unit, |mut header| {
                            header.col(|_ui| {});
                            header.col(|ui| {
                                ui.label("sn 1,3");
                            });
                            header.col(|ui| {
                                ui.label("sn 2");
                            });
                            header.col(|ui| {
                                ui.label("sn 1,2,3");
                            });
                            header.col(|ui| {
                                if ui.button("+").clicked() {
                                    value.insert(default(), default());
                                }
                            });
                        })
                        .body(|mut body| {
                            // value.retain2(|key, value| {
                            //     true
                            // });
                            for (fatty_acid, value) in value {
                                body.row(unit, |mut row| {
                                    row.col(|ui| {
                                        ui.text_edit_singleline(&mut fatty_acid.to_string());
                                    });
                                    for value in value {
                                        row.col(|ui| {
                                            ui.add(
                                                DragValue::new(value)
                                                    .clamp_range(0..=100)
                                                    .suffix('%'),
                                            );
                                        });
                                    }
                                    // row.col(|ui| {
                                    //     if ui.button("-").clicked() {
                                    //         value.insert(default(), default());
                                    //     }
                                    // });
                                });
                            }
                        });

                    // let mut count = 0;
                    // let mut percent = [0.0; 3];
                    // Grid::new("").num_columns(3).show(ui, |ui| {
                    //     for (fatty_acid, value) in value {
                    //         count += 1;
                    //         percent[0] += value[0];
                    //         percent[1] += value[1];
                    //         percent[2] += value[2];

                    //         ui.text_edit_singleline(&mut fatty_acid.to_string());
                    //         for value in &mut value[0..3] {
                    //             ui.add(
                    //                 DragValue::new(value).clamp_range(0..=100).suffix('%'),
                    //             );
                    //         }
                    //         // ui.label(format!("{:.1}%"suffix, value[0]))
                    //         //     .on_hover_text("sn-1,3");
                    //         // ui.label(format!("{:.1}%", value[1])).on_hover_text("sn-2");
                    //         // ui.label(format!("{:.1}%", value[2]))
                    //         //     .on_hover_text("sn-1,2,3");
                    //         ui.end_row();
                    //     }
                    //     if !self.info {
                    //         ui.separator();
                    //         ui.end_row();
                    //         ui.label(count.to_string())
                    //             .on_hover_text("Fatty acids count");
                    //         ui.label(format!("{:.1}%", percent[0]))
                    //             .on_hover_text("Fatty acids in sn 1,3 position");
                    //         ui.label(format!("{:.1}%", percent[1]))
                    //             .on_hover_text("Fatty acids in sn 2 position");
                    //         ui.label(format!("{:.1}%", percent[2]))
                    //             .on_hover_text("Fatty acids in sn 1,2,3 position");
                    //         ui.end_row();
                    //     }
                    // });
                })
                .header_response
                .on_hover_text(format!("{taxonomy:#}"))
                .context_menu(move |ui| {
                    // if ui.button("+").clicked() {
                    //     value.insert(default(), default());
                    // }
                });
        }
        response
    }
}

/// Table widget
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TableWidget {
    pub(crate) input: Input,
    pub(crate) inverted: bool,
    pub(crate) height: f32,
}

impl TableWidget {
    fn direct(&mut self, ui: &mut Ui) -> Response {
        let &mut Self { height, .. } = self;
        let taxonomies = self.input.taxonomies();
        let fatty_acids = self.input.fatty_acids();
        TableBuilder::new(ui)
            .resizable(true)
            .striped(true)
            .cell_layout(Layout::default().with_cross_align(Align::Center))
            .column(Size::relative(0.1))
            .columns(Size::remainder(), fatty_acids.len())
            .header(height * 3.0, |mut header| {
                header.col(|_ui| {});
                for fatty_acid in &fatty_acids {
                    header.col(|ui| {
                        StripBuilder::new(ui)
                            .sizes(Size::remainder(), 2)
                            .vertical(|mut strip| {
                                strip.cell(|ui| {
                                    ui.heading(fatty_acid);
                                });
                                strip.strip(|builder| {
                                    builder.sizes(Size::remainder(), 3).horizontal(|mut strip| {
                                        strip.cell(|ui| {
                                            ui.label("sn-1,3");
                                        });
                                        strip.cell(|ui| {
                                            ui.label("sn-2");
                                        });
                                        strip.cell(|ui| {
                                            ui.label("sn-1,2,3");
                                        });
                                    });
                                });
                            });
                    });
                }
            })
            .body(|mut body| {
                for taxonomy in &taxonomies {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.label(format!("{taxonomy}"))
                                .on_hover_text(format!("{taxonomy:#}"));
                        });
                        for fatty_acid in &fatty_acids {
                            row.col(|ui| {
                                ui.centered_and_justified(|ui| {
                                    if let Some(value) = self.input[taxonomy].get(fatty_acid) {
                                        StripBuilder::new(ui)
                                            .sizes(Size::remainder(), 3)
                                            .horizontal(|mut strip| {
                                                for value in &value[0..3] {
                                                    strip.cell(|ui| {
                                                        ui.label(format!("{value:03.1}"));
                                                    });
                                                }
                                            });
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            });
                        }
                    });
                }
            });
        ui.default_response()
    }

    fn inverted(&mut self, ui: &mut Ui) -> Response {
        let &mut Self { height, .. } = self;
        let taxonomies = self.input.taxonomies();
        let fatty_acids = self.input.fatty_acids();
        TableBuilder::new(ui)
            .resizable(true)
            .striped(true)
            .cell_layout(Layout::default().with_cross_align(Align::Center))
            .column(Size::exact(self.height * 2.0))
            .columns(
                Size::remainder()
                    .at_least(self.height * 2.0)
                    .at_most(self.height * 10.0),
                taxonomies.len(),
            )
            // .columns(Size::remainder(), taxonomies.len())
            .header(height * 1.5, |mut header| {
                header.col(|_ui| {});
                for taxonomy in &taxonomies {
                    header.col(|ui| {
                        ui.heading(taxonomy.to_string())
                            .on_hover_text(format!("{taxonomy:#}"));
                    });
                }
            })
            .body(|mut body| {
                for fatty_acid in &fatty_acids {
                    body.row(height, |mut row| {
                        row.col(|ui| {
                            ui.label(fatty_acid);
                        });
                        for taxonomy in &taxonomies {
                            row.col(|ui| {
                                if let Some(value) = self.input[taxonomy].get(fatty_acid) {
                                    Grid::new(format!("{taxonomy}{fatty_acid}")).show(ui, |ui| {
                                        ui.label(format!("{:.1}", value[0]))
                                            .on_hover_text("sn-1,3");
                                        ui.label(format!("{:.1}", value[1])).on_hover_text("sn-2");
                                        ui.label(format!("{:.1}", value[2]))
                                            .on_hover_text("sn-1,2,3");
                                        ui.end_row();
                                    });
                                } else {
                                    ui.label("-");
                                }
                            });
                        }
                    });
                }
            });
        ui.default_response()
    }
}

impl Widget for &mut TableWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        self.height = TextStyle::Body.resolve(ui.style()).size;
        if self.inverted {
            self.inverted(ui)
        } else {
            self.direct(ui)
        }
    }
}
