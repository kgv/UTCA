use crate::{
    app::Filter,
    config::{PositionalComposition, Sort},
    tag::Pattern,
    utils::{FloatExt, UiExt},
    Input, Output, Specie, Tag,
};
use egui::{Align, Grid, Layout, Response, ScrollArea, TextStyle, Ui, Widget};
use egui_extras::{Column, Size, TableBuilder};
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{default::default, ops::Bound};
use tracing::error;

/// Table UI
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Table {
    pub output: Output,
    pub filter: Filter,
}

impl Table {
    pub fn ui(&mut self, ui: &mut Ui) {
        let size = 1.5 * TextStyle::Body.resolve(ui.style()).size;
        let filtered = self.filter.filtered(self.output.clone());
        let species = filtered.species();
        let tags = filtered.compositions();
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
                            ui.heading(specie.to_string()).on_hover_ui(|ui| {
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
                    for &composition in &tags {
                        body.row(size, |mut row| {
                            row.col(|ui| {
                                if self.filter.positional_composition.is_none() {
                                    ui.label(composition.to_string());
                                } else {
                                    ui.label(format!("{composition:#}"));
                                }
                            });
                            for &specie in &species {
                                row.col(|ui| {
                                    if let Some(value) = filtered.0[specie].get(composition) {
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
