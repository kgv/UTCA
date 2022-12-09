use crate::{
    config::{PositionalComposition, Sort},
    taxonomy::{Specie, Taxonomy},
    utils::{FloatExt, UiExt},
    Input, Output, Triplet,
};
use egui::{Align, Grid, Layout, Response, ScrollArea, TextStyle, Ui, Widget};
use egui_extras::{Column, Size, TableBuilder};
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{default::default, ops::Bound};
use tracing::error;

/// Table widget
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct TableWidget {
    pub output: Output,
    pub bound: Bound<f64>,
    pub fatty_acids: [String; 3],
    pub positional_composition: Option<PositionalComposition>,
}

impl Default for TableWidget {
    fn default() -> Self {
        Self {
            output: default(),
            bound: Bound::Unbounded,
            fatty_acids: default(),
            positional_composition: default(),
        }
    }
}

impl TableWidget {
    fn filtered(&self) -> Output {
        self.output
            .clone()
            .bounded(self.bound)
            .positional_composition(self.positional_composition)
            .filter(|key, _| {
                (self.fatty_acids[0].is_empty() || self.fatty_acids[0] == key[0])
                    && (self.fatty_acids[1].is_empty() || self.fatty_acids[1] == key[1])
                    && (self.fatty_acids[2].is_empty() || self.fatty_acids[2] == key[2])
            })
            .sort(Sort::Key)
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        let size = 1.5 * TextStyle::Body.resolve(ui.style()).size;
        let filtered = self.filtered();
        let species = filtered.species();
        let triplets = filtered.triplets();
        ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
            TableBuilder::new(ui)
                .resizable(true)
                .vscroll(true)
                .striped(true)
                .cell_layout(Layout::centered_and_justified(egui::Direction::LeftToRight))
                .column(Column::auto().resizable(true))
                .columns(Column::remainder().at_least(4.0 * size), species.len())
                .header(size, |mut row| {
                    row.col(|_| {});
                    for &specie in &species {
                        row.col(|ui| {
                            ui.heading(specie.name()).on_hover_ui(|ui| {
                                // ui.heading(format!("ℹ {}", specie.taxonomy(".")));
                                Grid::new("").show(ui, |ui| {
                                    let triglycerides = &self.output[specie];
                                    let filtered = &filtered[specie];
                                    ui.heading("Minor");
                                    ui.heading("Major");
                                    ui.heading("∑");
                                    ui.end_row();
                                    let sum = triglycerides.len();
                                    let major = filtered.len();
                                    let minor = sum - major;
                                    ui.label(format!("{minor}"));
                                    ui.label(format!("{major}"));
                                    ui.label(format!("{sum}"));
                                    ui.end_row();
                                    let sum = triglycerides.values().sum::<f64>();
                                    let major = filtered.values().sum::<f64>();
                                    let minor = sum - major;
                                    ui.label(format!("{minor:.1}%"));
                                    ui.label(format!("{major:.1}%"));
                                    ui.label(format!("{sum:.1}%"));
                                    ui.end_row();
                                });
                            });
                        });
                    }
                })
                .body(|mut body| {
                    for &triplet in &triplets {
                        body.row(size, |mut row| {
                            row.col(|ui| {
                                if self.positional_composition.is_none() {
                                    ui.label(triplet.to_string());
                                } else {
                                    ui.label(format!("{triplet:#}"));
                                }
                            });
                            for &specie in &species {
                                row.col(|ui| {
                                    if let Some(value) = filtered.0[specie].get(triplet) {
                                        ui.label(format!("{value:.4}%"));
                                    } else {
                                        ui.label("-");
                                    }
                                });
                            }
                        });
                    }
                });
        });
    }
}
