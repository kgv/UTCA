use crate::{
    specie::Specie,
    utils::{CollapsingStateExt, IndexMapExt, UiExt},
    Input,
};
use egui::{
    collapsing_header::CollapsingState, CollapsingHeader, Context, Direction, Grid, Id, Layout,
    RichText, ScrollArea, TextStyle, Ui, Window,
};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use indexmap::{map::MutableKeys, IndexMap};
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, HashMap, HashSet},
    mem::take,
};
use tracing::error;

const SN: [&str; 3] = ["sn 1, 3", "sn 2", "sn 1, 2, 3"];

// fn temp(fatty_acids: &IndexMap<String, Vec<f64>>, fatty_acid: &String) {
//     let value = fatty_acids.shift_remove(fatty_acid)?;
//     let key = self.texts.remove(fatty_acid);
//     fatty_acids.insert(, value);
// }

/// List
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct List {
    pub input: Input,
    pub edit: bool,
    pub open: Option<bool>,

    removed: HashMap<Specie, Vec<String>>,
    selected: HashMap<Specie, bool>,
    size: f32,
    texts: IndexMap<Specie, HashMap<String, String>>,
}

impl List {
    pub fn ui(&mut self, ui: &mut Ui) {
        self.size = 1.5 * TextStyle::Body.resolve(ui.style()).size;
        ui.vertical_centered(|ui| ui.heading("Input"));
        ui.separator();
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                let open = self.open.take();
                for specie in &self.input.species() {
                    let remove = self.collapsing(ui, specie, open);
                    if remove {
                        self.input.remove(specie);
                    }
                }
                if self.edit && ui.button("+").on_hover_text("+ specie").clicked() {
                    self.input.insert(Specie::new(), IndexMap::new());
                }
            });
    }

    fn collapsing(&mut self, ui: &mut Ui, specie: &Specie, open: Option<bool>) -> bool {
        let selected = self.selected.entry(specie.clone()).or_default();
        CollapsingState::load_with_default_open(ui.ctx(), Id::new(specie), true)
            .open(open)
            .show_header(ui, |ui| {
                ui.toggle_value(selected, RichText::from(specie.to_string()).heading())
                    .on_hover_text(specie.taxonomy("."));
                self.edit && ui.button("-").on_hover_text("- specie").clicked()
            })
            .body(|ui| {
                self.table(ui, specie);
            })
            .1
            .inner
    }

    fn table(&mut self, ui: &mut Ui, specie: &Specie) {
        let &mut Self { size, .. } = self;
        TableBuilder::new(ui)
            .striped(true)
            .cell_layout(Layout::centered_and_justified(Direction::LeftToRight))
            .columns(Column::auto().resizable(true), 4)
            .column(Column::exact(size))
            .header(size, |mut row| {
                row.col(|_ui| {});
                for index in 0..3 {
                    row.col(|ui| {
                        ui.label(SN[index]);
                    });
                }
                row.col(|_ui| {});
            })
            .body(|mut body| {
                for fatty_acid in &self.input[specie].keys().cloned().collect::<Vec<_>>() {
                    body.row(size, |mut row| {
                        let mut lost_focus = false;
                        row.col(|ui| {
                            if self.edit {
                                let text = self
                                    .texts
                                    .entry(specie.clone())
                                    .or_default()
                                    .entry(fatty_acid.clone())
                                    .or_insert(fatty_acid.clone());
                                lost_focus = ui.text_edit_singleline(text).lost_focus();
                            } else {
                                ui.label(fatty_acid.to_string());
                            }
                        });
                        let values = &mut self.input[specie][fatty_acid];
                        // sn123 = (sn1 + sn2 + sn3) / 3.0 = (2.0 * sn13 + sn2) / 3.0
                        let sn123 = (2.0 * values[0] + values[1]) / 3.0;
                        // sn13 = (3.0 * sn123 - sn2) / 2.0
                        let sn13 = (3.0 * values[2] - values[1]) / 2.0;
                        // sn2 = 3.0 * sn123 - 2.0 * sn13
                        let sn2 = 3.0 * values[2] - 2.0 * values[0];
                        let calculated = [sn13, sn2, sn123];
                        for (index, value) in values.iter_mut().enumerate() {
                            row.col(|ui| {
                                if self.edit {
                                    ui.drag_percent(value)
                                } else {
                                    ui.label(format!("{value:05.2}%"))
                                }
                                .on_hover_text(format!("{:.2}%", calculated[index]));
                            });
                        }
                        row.col(|ui| {
                            if self.edit && ui.button("-").on_hover_text("- fatty acid").clicked() {
                                self.input[specie].remove(fatty_acid);
                            }
                        });
                        if lost_focus && &self.texts[specie][fatty_acid] != fatty_acid {
                            let text = self.texts[specie].remove(fatty_acid).unwrap();
                            if !self.input[specie].contains_key(&text) {
                                self.input[specie].replace(fatty_acid, text);
                            }
                        }
                    });
                }
                // + fatty acid
                if self.edit {
                    body.row(size, |mut row| {
                        row.col(|ui| {
                            if ui.button("+").on_hover_text("+ fatty acid").clicked() {
                                self.input[specie].insert(String::new(), vec![0.0, 0.0, 0.0]);
                            }
                        });
                    });
                }
                // ∑
                body.row(size, |mut row| {
                    row.col(|ui| {
                        ui.heading("∑");
                    });
                    let fatty_acids = &self.input[specie];
                    for index in 0..3 {
                        row.col(|ui| {
                            let sum = fatty_acids.values().map(|value| value[index]).sum::<f64>();
                            let count = fatty_acids.len();
                            ui.label(format!("{sum:.1}%"))
                                .on_hover_text(format!("{count}"));
                        });
                    }
                    row.col(|_ui| {});
                });
            });
    }
}
