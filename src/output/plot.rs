use crate::{utils::FloatExt, Config, Output};
use egui::{
    plot::{Bar, BarChart, Legend, LinkedAxisGroup, Plot as EguiPlot},
    ScrollArea, Ui,
};
use serde::{Deserialize, Serialize};
use std::ops::RangeInclusive;

/// Plot UI
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Plot {
    pub output: Output,
    pub config: Config,
    pub stacked: bool,
}

impl Plot {
    fn direct(&mut self, ui: &mut Ui) {
        let configured = self.output.clone().configure(&self.config);
        let species = configured.species();
        let tags = configured.tags();
        EguiPlot::new("plot")
            .x_axis_formatter(|x, _range: &RangeInclusive<f64>| {
                // let species = configured.species();
                // if let Some(specie) = (x as usize)
                //     .checked_sub(1)
                //     .and_then(|index| species.get(index))
                // {
                //     return specie.to_string();
                // }
                String::new()
            })
            .y_axis_formatter(|y, _range: &RangeInclusive<f64>| {
                if !y.is_approx_zero() && y.is_approx_integer() {
                    return format!("{y:.0}%");
                }
                String::new()
            })
            .legend(Legend::default())
            .show(ui, |plot_ui| {
                let mut offsets = vec![0.0; species.len()];
                for &tags in &tags {
                    let mut bars = Vec::new();
                    for (index, &specie) in species.iter().enumerate() {
                        if let Some(&value) = configured[specie].get(tags) {
                            let mut bar = Bar::new(1.0 + index as f64, value)
                                .name(format!("{specie}\n{tags}"));
                            if self.stacked {
                                bar = bar.base_offset(offsets[index]);
                            }
                            bars.push(bar);
                            offsets[index] += value;
                        }
                    }
                    let chart = BarChart::new(bars).width(0.75).name(tags.to_string());
                    plot_ui.bar_chart(chart);
                }
            });
    }

    // fn inverted(&mut self, ui: &mut Ui) {
    //     let configured = self.output.clone().configure(&self.config);
    //     let species = configured.species();
    //     let tags = configured.tags();
    //     let plot = EguiPlot::new("plot")
    //         .x_axis_formatter(move |x, _range: &RangeInclusive<f64>| {
    //             // if let Some(key) = (x as usize)
    //             //     .checked_sub(1)
    //             //     .and_then(|index| keys.get(index))
    //             // {
    //             //     return key.to_string();
    //             // }
    //             String::new()
    //         })
    //         .y_axis_formatter(|y, _range: &RangeInclusive<f64>| {
    //             if !y.is_approx_zero() && y.is_approx_integer() {
    //                 return format!("{y:.0}%");
    //             }
    //             String::new()
    //         })
    //         .legend(Legend::default());
    //     plot.show(ui, |plot_ui| {
    //         for (index, &specie) in species.iter().enumerate() {
    //             let mut offset = 0.0;
    //             let mut bars = Vec::new();
    //             for &tags in &tags {
    //                 if let Some(&value) = configured[specie].get(tags) {
    //                     let bar = Bar::new(1.0 + index as f64, value)
    //                         .name(tags.to_string())
    //                         .base_offset(offset);
    //                     bars.push(bar);
    //                     offset += value;
    //                 }
    //             }
    //             let chart = BarChart::new(bars).width(0.75).name(specie.to_string());
    //             plot_ui.bar_chart(chart);
    //         }
    //     });
    // }

    fn splited(&mut self, ui: &mut Ui) {
        let configured = self.output.clone().configure(&self.config);
        let species = configured.species();
        let tags = configured.tags();
        let group = LinkedAxisGroup::new(true, false);
        ScrollArea::vertical()
            .auto_shrink([false; 2])
            .show(ui, |ui| {
                for &tags in &tags {
                    EguiPlot::new(tags)
                        .height(256.0)
                        .data_aspect(1.0)
                        .legend(Legend::default())
                        .link_axis(group.clone())
                        .show(ui, |ui| {
                            let bars = species
                                .iter()
                                .enumerate()
                                .filter_map(|(index, &specie)| {
                                    let &value = configured[specie].get(tags)?;
                                    let bar = Bar::new(1.0 + index as f64, value)
                                        .name(format!("{specie}\n{tags}"));
                                    Some(bar)
                                })
                                .collect();
                            let chart = BarChart::new(bars).width(0.75).name(tags.to_string());
                            ui.bar_chart(chart);
                            // for (index, &specie) in species.iter().enumerate() {
                            //     let mut bars = Vec::new();
                            //     if let Some(&value) = configured[specie].get(tags) {
                            //         let bar = Bar::new(1.0 + index as f64, value).name(tags.to_string());
                            //         bars.push(bar);
                            //     }
                            //     let chart = BarChart::new(bars).width(0.75).name(specie.to_string());
                            //     ui.bar_chart(chart);
                            // }
                        });
                }
            });
    }

    pub fn ui(&mut self, ui: &mut Ui) {
        // self.direct(ui);
        self.splited(ui);

        // if self.stacked {
        //     self.inverted(ui);
        // } else {
        // }
    }
}
