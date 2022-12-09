use crate::{taxonomy::Specie, Taxonomy, Triplet};
use egui::{
    plot::{Bar, BarChart, Legend, Plot},
    Align, CollapsingHeader, Grid, Layout, Response, RichText, ScrollArea, TextStyle, Ui, Widget,
    Window,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};

/// Plot widget
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct PlotWidget {
    pub(crate) data: IndexMap<Specie, IndexMap<Triplet, f64>>,
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
