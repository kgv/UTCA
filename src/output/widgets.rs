use super::Output;
use crate::{config::Sort, triplet, utils::FloatExt, Input, Taxonomy, Triplet};
use egui::{
    plot::{Bar, BarChart, Legend, Plot},
    Align, CollapsingHeader, Grid, Layout, Response, ScrollArea, TextStyle, Ui, Widget,
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
    ops::{Bound, RangeInclusive},
};
use tracing::{error, warn};

/// List widget
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct ListWidget {
    pub(crate) data: Output,
    pub(crate) expand: Option<bool>,
}

impl Widget for &mut ListWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        CollapsingHeader::new("Composition")
            .default_open(true)
            .show(ui, |ui| {
                error!("{:?}", &self.expand);
                for (key, value) in &self.data {
                    CollapsingHeader::new(format!("{key}"))
                        .open(self.expand)
                        .show(ui, |ui| {
                            Grid::new("composition_grid").show(ui, |ui| {
                                let mut count = 0;
                                let mut sum = 0.0;
                                for (key, &value) in value {
                                    ui.label(format!("{key}"));
                                    ui.label(format!("{value:.4}"));
                                    ui.end_row();
                                    count += 1;
                                    sum += value;
                                }
                                ui.separator();
                                ui.end_row();
                                ui.label(format!("{count}"));
                                ui.label(format!("{sum:.1}%"));
                                ui.end_row();
                            });
                        })
                        .header_response
                        .on_hover_text(format!("{key:#}"));
                }
            })
            .header_response
            .context_menu(|ui| {
                self.expand = None;
                if ui.button("Expand all").clicked() {
                    self.expand = Some(true);
                    warn!("{:?}", &self.expand);
                    ui.close_menu();
                }
                if ui.button("Collapse all").clicked() {
                    self.expand = Some(false);
                    warn!("{:?}", &self.expand);
                    ui.close_menu();
                }
            })
    }
}

/// Table widget
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct TableWidget {
    pub(crate) data: IndexMap<Taxonomy, IndexMap<Triplet, f64>>,
    pub(crate) inverted: bool,
    pub(crate) text_height: f32,
}

impl TableWidget {
    fn direct(&mut self, ui: &mut Ui) -> Response {
        let response = ui.heading("");

        let &mut Self { text_height, .. } = self;
        let mut columns = self.internal();
        columns.sort();
        let mut rows = self.external();
        rows.sort();
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::default().with_cross_align(Align::Center))
            .column(Size::relative(0.1))
            .columns(Size::remainder().at_least(4.0 * text_height), columns.len())
            .header(1.5 * text_height, |mut header| {
                header.col(|_| {});
                for &key in &columns {
                    header.col(|ui| {
                        ui.heading(key.to_string());
                    });
                }
            })
            .body(|mut body| {
                for &key in &rows {
                    body.row(text_height, |mut row| {
                        row.col(|ui| {
                            ui.label(key.to_string()).on_hover_text(format!("{key:#}"));
                        });
                        let value = self.data.get(key);
                        for &key in &columns {
                            row.col(|ui| {
                                if let Some(value) = value.and_then(|value| value.get(key)) {
                                    ui.label(format!("{value:.4}"));
                                } else {
                                    ui.label("-");
                                }
                            });
                        }
                    });
                }
            });
        response
    }

    fn external(&self) -> Vec<&Taxonomy> {
        self.data.keys().collect()
    }

    fn internal(&self) -> Vec<&Triplet> {
        self.data
            .values()
            .flat_map(IndexMap::keys)
            .unique()
            .collect()
    }

    fn inverted(&mut self, ui: &mut Ui) -> Response {
        let response = ui.heading("");

        let &mut Self { text_height, .. } = self;
        let mut columns = self.external();
        columns.sort();
        let mut rows = self.internal();
        rows.sort();
        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(Layout::default().with_cross_align(Align::Center))
            .column(Size::relative(0.1))
            .columns(Size::remainder().at_least(4.0 * text_height), columns.len())
            .header(1.5 * text_height, |mut header| {
                header.col(|_| {});
                for column in &columns {
                    header.col(|ui| {
                        ui.heading(column.to_string())
                            .on_hover_text(format!("{column:#}"));
                    });
                }
            })
            .body(|mut body| {
                for &key in &rows {
                    let k1 = key;
                    // let value = |value: &IndexMap<Triplet, f64>| value.get(key);
                    body.row(text_height, |mut row| {
                        row.col(|ui| {
                            ui.label(key.to_string());
                        });
                        for &key in &columns {
                            row.col(|ui| {
                                if let Some(value) =
                                    self.data.get(key).and_then(|value| value.get(k1))
                                {
                                    ui.label(format!("{value:.4}"));
                                } else {
                                    ui.label("-");
                                }
                            });
                        }
                    });
                }
            });
        response
    }
}

impl Widget for &mut TableWidget {
    fn ui(self, ui: &mut Ui) -> Response {
        self.text_height = TextStyle::Body.resolve(ui.style()).size;
        if self.inverted {
            self.inverted(ui)
        } else {
            self.direct(ui)
        }
    }
}

/// Plot widget
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlotWidget {
    pub(crate) data: IndexMap<Taxonomy, IndexMap<Triplet, f64>>,
    pub(crate) inverted: bool,
}

// impl PlotWidget {
//     fn direct(&mut self, ui: &mut Ui) -> Response {
//         // let keys = self.data.direct.keys();
//         let plot = Plot::new("plot")
//             .x_axis_formatter(move |x, _range: &RangeInclusive<f64>| {
//                 // if let Some(key) = (x as usize)
//                 //     .checked_sub(1)
//                 //     .and_then(|index| keys.get(index))
//                 // {
//                 //     return key.to_string();
//                 // }
//                 String::new()
//             })
//             .y_axis_formatter(|y, _range: &RangeInclusive<f64>| {
//                 if !y.is_approx_zero() && y.is_approx_integer() {
//                     return format!("{y:.0}%");
//                 }
//                 String::new()
//             })
//             .legend(Legend::default());
//         plot.show(ui, |plot_ui| {
//             for (index, (key, value)) in self.data.direct.iter().enumerate() {
//                 let mut offset = 0.0;
//                 let bars = value
//                     .iter()
//                     // .sorted_by(|a, b| match self.sort {
//                     //     Sort::Key => b.0.cmp(a.0),
//                     //     Sort::Value => b.1.total_cmp(a.1),
//                     // })
//                     .map(|(key, &value)| {
//                         let bar = Bar::new(1.0 + index as f64, value)
//                             .name(key)
//                             .base_offset(offset);
//                         offset += value;
//                         bar
//                     })
//                     .collect();
//                 let chart = BarChart::new(bars).width(0.75).name(key.to_string());
//                 plot_ui.bar_chart(chart);
//             }
//         })
//         .response
//     }

//     fn inverted(&mut self, ui: &mut Ui) -> Response {
//         let plot = Plot::new("plot")
//             .x_axis_formatter(move |x, _range: &RangeInclusive<f64>| {
//                 // if let Some(key) = (x as usize)
//                 //     .checked_sub(1)
//                 //     .and_then(|index| keys.get(index))
//                 // {
//                 //     return key.to_string();
//                 // }
//                 String::new()
//             })
//             .y_axis_formatter(|y, _range: &RangeInclusive<f64>| {
//                 if !y.is_approx_zero() && y.is_approx_integer() {
//                     return format!("{y:.0}%");
//                 }
//                 String::new()
//             })
//             .legend(Legend::default());
//         plot.show(ui, |plot_ui| {
//             for (index, (key, value)) in self.data.direct.iter().enumerate() {
//                 let mut offset = 0.0;
//                 let bars = value
//                     .iter()
//                     // .sorted_by(|a, b| match self.sort {
//                     //     Sort::Key => b.0.cmp(a.0),
//                     //     Sort::Value => b.1.total_cmp(a.1),
//                     // })
//                     .map(|(key, &value)| {
//                         let bar = Bar::new(1.0 + index as f64, value)
//                             .name(key)
//                             .base_offset(offset);
//                         offset += value;
//                         bar
//                     })
//                     .collect();
//                 let chart = BarChart::new(bars).width(0.75).name(key.to_string());
//                 plot_ui.bar_chart(chart);
//             }
//         })
//         .response
//     }
// }

// impl Widget for &mut PlotWidget {
//     fn ui(self, ui: &mut Ui) -> Response {
//         if self.inverted {
//             self.inverted(ui)
//         } else {
//             self.direct(ui)
//         }
//     }
// }
