use crate::{
    app::Filter,
    config::{PositionalComposition, Sort},
    tag::{self, Composition, Pattern},
    utils::{FloatExt, UiExt},
    Input, Output, Specie, Tag,
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

/// List UI
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct List {
    pub output: Output,

    pub filter: Filter,
    pub expand: Option<bool>,
}

impl List {
    pub fn ui(&mut self, ui: &mut Ui) {
        let filtered = self.filter.filtered(self.output.clone());
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for (specie, value) in &filtered {
                    CollapsingHeader::new(RichText::from(specie.to_string()).heading())
                        .open(self.expand)
                        .show(ui, |ui| {
                            Grid::new("").show(ui, |ui| {
                                for (composition, &value) in value {
                                    if self.filter.positional_composition.is_none() {
                                        ui.label(composition.to_string());
                                    } else {
                                        ui.label(format!("{composition:#}"));
                                    }
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
}

// pub fn windows(&mut self, ctx: &Context) {
//     // let statistic = self.windows.statistic;
//     if !self.output.is_empty() && !self.windows.statistic.specie.is_empty() {
//         let triglycerides = &self.output[&self.windows.statistic.specie];
//         let filtered = &self.filtered()[&self.windows.statistic.specie];
//         Window::new(format!("ℹ {}", self.windows.statistic.specie.name()))
//             // .current_pos(self.left_top)
//             .default_pos(self.center_top)
//             .open(&mut self.windows.statistic.open)
//             .show(ctx, |ui| {
//                 ui.heading("Count");
//                 let major = filtered.len();
//                 let minor = triglycerides.len() - major;
//                 ui.label(format!("Major: {major}"));
//                 ui.label(format!("Minor: {minor}"));
//                 ui.heading("Percent");
//                 let major = filtered.values().sum::<f64>();
//                 let minor = triglycerides.values().sum::<f64>() - major;
//                 ui.label(format!("Major: {major:.1}%",));
//                 ui.label(format!("Minor: {minor:.1}%",));
//             });
//     }
// }
