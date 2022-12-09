use crate::{
    config::Sort,
    taxonomy::Specie,
    triplet,
    utils::{FloatExt, UiExt},
    Input, Output, Taxonomy, Triplet,
};
use egui::{
    collapsing_header::CollapsingState,
    plot::{Bar, BarChart, Legend, Plot},
    pos2, Align, CollapsingHeader, Context, Grid, Id, Layout, Pos2, Response, RichText, ScrollArea,
    TextStyle, Ui, Widget, Window,
};
use egui_extras::{Size, TableBuilder};
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{
    cmp::Ordering,
    collections::{BTreeMap, HashMap},
    default::default,
    fmt::Display,
    hash::Hash,
    ops::{Bound, RangeInclusive, Sub},
};
use tracing::{error, warn};

/// List widget
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct ListWidget {
    pub input: Input,
    pub output: Output,

    pub bound: Bound<f64>,
    pub fatty_acids: [String; 3],
    pub expand: Option<bool>,
    pub sort: Sort,
    pub center_top: Pos2,

    windows: Windows,
}

impl Default for ListWidget {
    fn default() -> Self {
        Self {
            input: default(),
            output: default(),
            bound: Bound::Unbounded,
            fatty_acids: default(),
            expand: default(),
            sort: default(),
            center_top: default(),
            windows: default(),
        }
    }
}

impl ListWidget {
    fn filtered(&self) -> Output {
        self.output
            .clone()
            .bounded(self.bound)
            .filter(|key, _| {
                (self.fatty_acids[0].is_empty() || self.fatty_acids[0] == key[0])
                    && (self.fatty_acids[1].is_empty() || self.fatty_acids[1] == key[1])
                    && (self.fatty_acids[2].is_empty() || self.fatty_acids[2] == key[2])
            })
            .sort(self.sort)
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        self.center_top = ui.cursor().center_top();
        let filtered = self.filtered();
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (specie, value) in &filtered {
                    CollapsingHeader::new(RichText::from(specie.name()).heading())
                        .open(self.expand)
                        .show(ui, |ui| {
                            Grid::new("").show(ui, |ui| {
                                for (key, &value) in value {
                                    ui.label(format!("{key}")).on_hover_text(format!("{key:#}"));
                                    ui.label(format!("{value:.4}%"))
                                        .on_hover_text(value.to_string());
                                    ui.end_row();
                                }
                            });
                        })
                        .header_response
                        .on_hover_ui(|ui| {
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
                }
            });
    }

    pub fn windows(&mut self, ctx: &Context) {
        // let statistic = self.windows.statistic;
        if !self.output.is_empty() && !self.windows.statistic.specie.is_empty() {
            let triglycerides = &self.output[&self.windows.statistic.specie];
            let filtered = &self.filtered()[&self.windows.statistic.specie];
            Window::new(format!("ℹ {}", self.windows.statistic.specie.name()))
                // .current_pos(self.left_top)
                .default_pos(self.center_top)
                .open(&mut self.windows.statistic.open)
                .show(ctx, |ui| {
                    ui.heading("Count");
                    let major = filtered.len();
                    let minor = triglycerides.len() - major;
                    ui.label(format!("Major: {major}"));
                    ui.label(format!("Minor: {minor}"));
                    ui.heading("Percent");
                    let major = filtered.values().sum::<f64>();
                    let minor = triglycerides.values().sum::<f64>() - major;
                    ui.label(format!("Major: {major:.1}%",));
                    ui.label(format!("Minor: {minor:.1}%",));
                });
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Windows {
    statistic: Statistic,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Statistic {
    open: bool,
    specie: Vec<String>,
}
