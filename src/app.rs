use crate::{
    config::{Io, Sort, Widget},
    input::{ListWidget as InputListWidget, TableWidget as InputTableWidget},
    utils::{BoundExt, Info},
    Input, Taxonomy, Visitor,
};
use anyhow::{Error, Result};
use eframe::{get_value, set_value, CreationContext, Frame, Storage, APP_KEY};
use egui::{
    menu, warn_if_debug_build, Align, Align2, CentralPanel, Color32, ComboBox, Context, Direction,
    DragValue, DroppedFile, FontId, HoveredFile, Id, LayerId, Layout, Order, Pos2, ScrollArea,
    SidePanel, TopBottomPanel, Widget as _,
};
use indexmap::IndexMap;
use serde::{Deserialize, Serialize};
use std::{default::default, ops::Bound, str};
use toml_edit::Document;
use tracing::error;

const HEADER: &str = "Positional-species and positional-type (SUS, SUU, UUU) composition of TAG from mature fruit arils of the Euonymus section species, mol % of total TAG";

fn parse(bytes: &[u8]) -> Result<IndexMap<Taxonomy, IndexMap<String, Vec<f64>>>> {
    let content = str::from_utf8(bytes)?;
    let document = content.parse::<Document>()?;
    Ok(Visitor::visit(&document))
}

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    files: Vec<DroppedFile>,
    input: Input,
    io: Io,
    widget: Widget,
    inverted: bool,
    sort: Sort,

    available_fatty_acids: Vec<String>,
    current_fatty_acids: [String; 3],
    bound: Bound<f64>,
    changed: bool,

    #[serde(skip)]
    img_offset: Pos2,
    #[serde(skip)]
    available_height: f32,

    #[serde(skip)]
    errors: Vec<Error>,
}

impl Default for App {
    fn default() -> Self {
        Self {
            files: default(),
            input: default(),
            io: default(),
            widget: default(),
            inverted: default(),
            sort: default(),

            available_fatty_acids: default(),
            current_fatty_acids: default(),
            bound: Bound::Unbounded,
            changed: true,

            img_offset: default(),
            available_height: default(),

            errors: default(),
        }
    }
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &CreationContext) -> Self {
        // This is also where you can customized the look at feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        if let Some(storage) = cc.storage {
            return get_value(storage, eframe::APP_KEY).unwrap_or_default();
        }
        default()
    }

    fn detect_input(&mut self, ctx: &Context) {
        // Preview hovered files
        if !ctx.input().raw.hovered_files.is_empty() {
            let mut text = "Dropping files:".to_owned();
            for file in &ctx.input().raw.hovered_files {
                text.push('\n');
                text += &file.info();
            }
            let painter =
                ctx.layer_painter(LayerId::new(Order::Foreground, Id::new("file_drop_target")));
            let screen_rect = ctx.input().screen_rect();
            painter.rect_filled(screen_rect, 0.0, Color32::from_black_alpha(192));
            painter.text(
                screen_rect.center(),
                Align2::CENTER_CENTER,
                text,
                FontId::monospace(14.0),
                Color32::WHITE,
            );
        }

        // Save dropped files
        if !ctx.input().raw.dropped_files.is_empty() {
            self.files = ctx.input().raw.dropped_files.clone();
        }
    }

    fn parse_input(&mut self) {
        if !self.files.is_empty() && self.input.is_empty() {
            for file in &self.files {
                if let Some(bytes) = &file.bytes {
                    match parse(bytes) {
                        Ok(input) => self.input = Input::new(input),
                        Err(err) => self.errors.push(err),
                    }
                }
            }
        }
        if !self.input.is_empty() {
            self.available_fatty_acids = self.input.fatty_acids();
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn Storage) {
        set_value(storage, APP_KEY, self);
    }

    /// Called each time the UI needs repainting, which may be many times per
    /// second.
    fn update(&mut self, ctx: &Context, _frame: &mut Frame) {
        #[cfg(not(target_arch = "wasm32"))] // no File->Quit on web pages!
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:
            menu::bar(ui, |ui| {
                ui.menu_button("File", |ui| {
                    if ui.button("Quit").clicked() {
                        _frame.close();
                    }
                });
            });
        });

        SidePanel::left("left_panel").show(ctx, |ui| {
            ui.heading("Control Panel");
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.io, Io::Input, "Input");
                ui.selectable_value(&mut self.io, Io::Output, "Output");
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.selectable_value(&mut self.widget, Widget::List, "List");
                ui.selectable_value(&mut self.widget, Widget::Table, "Table");
                ui.selectable_value(&mut self.widget, Widget::Plot, "Plot");
            });
            if let Widget::Table = self.widget {
                ui.separator();
                let text = if self.inverted { "🔃" } else { "🔄" };
                ui.toggle_value(&mut self.inverted, text);
            }
            let text = match self.sort {
                Sort::Key => "Sort by key",
                Sort::Value => "Sort by value",
            };
            ComboBox::from_label("")
                .selected_text(text)
                .show_ui(ui, |ui| {
                    ui.selectable_value(&mut self.sort, Sort::Key, "Sort by key");
                    ui.selectable_value(&mut self.sort, Sort::Value, "Sort by value");
                });
            // Filter
            ui.separator();
            ui.group(|ui| {
                ui.heading("Filter");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Key:")
                        .on_hover_text("Key filter by fatty acids in sn positions");
                    for (index, current) in self.current_fatty_acids.iter_mut().enumerate() {
                        ComboBox::new(format!("fatty_acids_combo_box_{index}"), "")
                            .selected_text(&*current)
                            .show_ui(ui, |ui| {
                                ui.selectable_value(current, String::new(), "None");
                                for fatty_acid in self.available_fatty_acids.clone() {
                                    ui.selectable_value(current, fatty_acid.clone(), fatty_acid);
                                }
                            });
                    }
                });
                ui.separator();
                ui.horizontal(|ui| {
                    ui.label("Value:")
                        .on_hover_text("Value filter by bound in percent");
                    let value = self.bound.value().copied().unwrap_or_default();
                    self.changed |= ComboBox::new("bound_combo_box", "")
                        .selected_text(self.bound.variant_name())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.bound,
                                Bound::Included(value),
                                "Included",
                            );
                            ui.selectable_value(
                                &mut self.bound,
                                Bound::Excluded(value),
                                "Excluded",
                            );
                            ui.selectable_value(&mut self.bound, Bound::Unbounded, "Unbounded");
                        })
                        .response
                        .changed();
                    if let Bound::Included(bound) | Bound::Excluded(bound) = &mut self.bound {
                        self.changed |= ui
                            .add(
                                DragValue::new(bound)
                                    .clamp_range(0.0..=100.0)
                                    .speed(0.1)
                                    .suffix("%"),
                            )
                            .changed();
                    }
                });
            });
            ui.separator();
            ui.group(|ui| {
                ui.heading("Statistics");
                ui.separator();
            });

            ui.with_layout(
                Layout::bottom_up(Align::Center).with_cross_align(Align::LEFT),
                |ui| {
                    warn_if_debug_build(ui);
                    if !self.files.is_empty() {
                        self.files.retain(|file| {
                            let mut remove = false;
                            ui.horizontal(|ui| {
                                ui.label(file.info());
                                if ui.button("❌").clicked() {
                                    remove = true;
                                }
                            });
                            !remove
                        });
                    }
                    ui.separator();
                    ui.heading("Files");
                },
            );
        });

        CentralPanel::default().show(ctx, |ui| {
            // Get available space for the image
            self.img_offset = ui.cursor().left_top();
            self.available_height = ui.available_height();

            if self.files.is_empty() {
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        ui.label("Drag and drop input file here...");
                    },
                );
            } else if !self.input.is_empty() {
                ui.heading(HEADER);
                ui.separator();
                ScrollArea::both()
                    .auto_shrink([false; 2])
                    .show(ui, |ui| match self.io {
                        Io::Input => {
                            let input = &mut self.input;
                            match self.widget {
                                Widget::List => {
                                    ui.add(&mut input.list());
                                }
                                Widget::Plot => {}
                                Widget::Table => {
                                    ui.add(&mut input.table(self.inverted));
                                }
                            }
                        }
                        Io::Output => {
                            let output = self
                                .input
                                .output()
                                .bounded(self.bound)
                                .filter(|key, _| {
                                    (self.current_fatty_acids[0].is_empty()
                                        || self.current_fatty_acids[0] == key[0])
                                        && (self.current_fatty_acids[1].is_empty()
                                            || self.current_fatty_acids[1] == key[1])
                                        && (self.current_fatty_acids[2].is_empty()
                                            || self.current_fatty_acids[2] == key[2])
                                })
                                .sort(self.sort);
                            match self.widget {
                                Widget::List => {
                                    ui.add(&mut output.list());
                                }
                                Widget::Plot => {
                                    // ui.add(&mut output.plot(self.inverted));
                                }
                                Widget::Table => {
                                    ui.add(&mut output.table(self.inverted));
                                }
                            }
                        }
                    });
            }
        });

        self.detect_input(ctx);
        self.parse_input();
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Widgets {
    input: InputWidgets,
    // output: OutputWidgets,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct InputWidgets {
    list: InputListWidget,
    // table: InputTableWidget<T>,
}

// struct OutputWidgets {
// }

// IndexMap<T, IndexMap<U, V>>
// pub(crate) struct Invertible<T, U, V> {
//     direct: IndexMap<T, IndexMap<U, V>>,
//     inverted: IndexMap<T, IndexMap<U, V>>,
// }

// pub(crate) trait Invert<T, U, V> {
//     fn invert(self) -> IndexMap<U, IndexMap<T, V>>;
// }

// pub fn invert(self) -> Input<IndexMap<String, IndexMap<Taxonomy, Vec<f64>>>> {
//     let mut inverted = IndexMap::new();
//     for (external_key, value) in self.0 {
//         for (internal_key, value) in value {
//             inverted
//                 .entry(internal_key.clone())
//                 .or_insert(IndexMap::new())
//                 .insert(external_key, value);
//         }
//     }
//     Input(inverted)
// }
