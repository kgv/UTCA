use std::{default::default, ops::RangeInclusive, slice};

use crate::{app::Filter, utils::FloatExt, Output, Specie, Tag};
use egui::{
    plot::{Bar, BarChart, Legend, Plot as EguiPlot},
    Align, CollapsingHeader, Grid, Layout, Response, RichText, ScrollArea, TextStyle, Ui, Widget,
    Window,
};
use indexmap::IndexMap;
use itertools::Itertools;
use serde::{Deserialize, Serialize};

/// Plot UI
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Plot {
    pub output: Output,

    pub filter: Filter,
    pub inverted: bool,
}

impl Plot {
    fn direct(&mut self, ui: &mut Ui) {
        let filtered = self.filter.filtered(self.output.clone());
        let species = filtered.species();
        let compositions = filtered.compositions();
        let plot = EguiPlot::new("plot")
            .x_axis_formatter(move |x, _range: &RangeInclusive<f64>| {
                // if let Some(key) = (x as usize)
                //     .checked_sub(1)
                //     .and_then(|index| keys.get(index))
                // {
                //     return key.to_string();
                // }
                String::new()
            })
            .y_axis_formatter(|y, _range: &RangeInclusive<f64>| {
                if !y.is_approx_zero() && y.is_approx_integer() {
                    return format!("{y:.0}%");
                }
                String::new()
            })
            .legend(Legend::default());
        plot.show(ui, |plot_ui| {
            for (index, &specie) in species.iter().enumerate() {
                let mut offset = 0.0;
                let mut bars = Vec::new();
                for &composition in &compositions {
                    if let Some(&value) = filtered[specie].get(composition) {
                        let bar = Bar::new(1.0 + index as f64, value)
                            .name(composition.to_string())
                            .base_offset(offset);
                        bars.push(bar);
                        offset += value;
                    }
                }
                let chart = BarChart::new(bars).width(0.75).name(specie.to_string());
                plot_ui.bar_chart(chart);
            }
        });
    }

    fn inverted(&mut self, ui: &mut Ui) {
        let filtered = self.filter.filtered(self.output.clone());
        let species = filtered.species();
        let compositions = filtered.compositions();
        let plot = EguiPlot::new("plot")
            .x_axis_formatter(move |x, _range: &RangeInclusive<f64>| {
                // if let Some(key) = (x as usize)
                //     .checked_sub(1)
                //     .and_then(|index| keys.get(index))
                // {
                //     return key.to_string();
                // }
                String::new()
            })
            .y_axis_formatter(|y, _range: &RangeInclusive<f64>| {
                if !y.is_approx_zero() && y.is_approx_integer() {
                    return format!("{y:.0}%");
                }
                String::new()
            })
            .legend(Legend::default());
        plot.show(ui, |plot_ui| {
            // let mut charts = Vec::new();
            for &composition in &compositions {
                let mut offsets = vec![0.0; species.len()];
                let mut bars = Vec::new();
                for (index, &specie) in species.iter().enumerate() {
                    if let Some(&value) = filtered[specie].get(composition) {
                        let bar = Bar::new(1.0 + index as f64, value)
                            .name(format!("{specie}\n{composition}"))
                            .base_offset(offsets[index]);
                        bars.push(bar);
                        offsets[index] = offsets[index] + value;
                    }
                }
                let chart = BarChart::new(bars)
                    .width(0.75)
                    .name(composition.to_string());
                plot_ui.bar_chart(chart);
            }
            // for chart in charts {
            //     charts.push(chart);
            // }

            // let mut charts = Vec::new();
            // for &composition in &compositions {
            //     let mut bars = Vec::new();
            //     // let mut offsets = vec![0.0; species.len()];
            //     for (index, &specie) in species.iter().enumerate() {
            //         if let Some(&value) = filtered[specie].get(composition) {
            //             let bar = Bar::new(1.0 + index as f64, value)
            //                 .name(format!("{specie}\n{composition}"));
            //             bars.push(bar);
            //         }
            //     }
            //     let chart = BarChart::new(bars)
            //         .stack_on(&charts.iter().collect::<Vec<_>>())
            //         .width(0.75)
            //         .name(composition.to_string());
            //     charts.push(chart);
            // }
            // for chart in charts {
            //     plot_ui.bar_chart(chart);
            // }
        });
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        if self.inverted {
            self.inverted(ui);
        } else {
            self.direct(ui);
        }
    }
}
