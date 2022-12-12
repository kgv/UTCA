use crate::{
    config::{Io, PositionalComposition, Sort, Widget},
    input::{List as InputList, Text as InputText},
    output::{List as OutputList, Plot as OutputPlot, Table as OutputTable},
    tag::Pattern,
    utils::{BoundExt, Info, UiExt},
    Input, Output, Visitor,
};
use anyhow::{Context as _, Error, Result};
use eframe::{get_value, set_value, CreationContext, Frame, Storage, APP_KEY};
use egui::{
    global_dark_light_mode_switch, menu, warn_if_debug_build, Align, Align2, CentralPanel, Color32,
    ComboBox, Context, Direction, DroppedFile, FontId, Id, LayerId, Layout, Order, Pos2,
    ScrollArea, SidePanel, TextStyle, TopBottomPanel, Window,
};
use egui_file::FileDialog;
use egui_notify::Toasts;
use itertools::Itertools;
use serde::{Deserialize, Serialize};
use std::{default::default, mem::MaybeUninit, ops::Bound, slice, str, time::Duration};
use toml_edit::Document;
use tracing::error;

// 🔍🔧📝🖹⚙🛠⬇🔃🔄
const HEADER: &str = "Positional-species and positional-type (SUS, SUU, UUU) composition of TAG from mature fruit arils of the Euonymus section species, mol % of total TAG";

fn parse_input(files: &[DroppedFile]) -> Result<Input> {
    let file = &files[0];
    let bytes = file
        .bytes
        .as_ref()
        .with_context(|| "Dropped file bytes is none")?;
    let content = str::from_utf8(bytes)?;
    let document = content.parse::<Document>()?;
    Ok(Input::new(Visitor::visit(&document)))
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    files: Vec<DroppedFile>,
    input: Option<Input>,

    left_panel: bool,
    io: Io,
    widget: Widget,
    available_fatty_acids: Vec<String>,
    filter: Filter,

    ui: Ui,
    widgets: Widgets,
    #[serde(skip)]
    toasts: Toasts,
    #[serde(skip)]
    errors: Vec<Error>,
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

    fn file_drag_and_drop_ui(&mut self, ctx: &egui::Context) {
        // Preview hovering files
        if !ctx.input().raw.hovered_files.is_empty() {
            let mut text = "Dropping files:\n".to_owned();
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
                TextStyle::Heading.resolve(&ctx.style()),
                Color32::WHITE,
            );
        }

        // Collect dropped files
        if !ctx.input().raw.dropped_files.is_empty() {
            self.files = ctx.input().raw.dropped_files.clone();
            match parse_input(&self.files) {
                Ok(input) => {
                    self.available_fatty_acids = input.fatty_acids();
                    // self.ui.text.text = input.output();
                    self.widgets.output.list.output = input.output();
                    self.widgets.output.plot.output = input.output();
                    self.widgets.output.table.output = input.output();
                    self.input = Some(input);
                }
                Err(err) => {
                    error!(%err);
                    self.toasts
                        .error(format!("{err:?}"))
                        .set_duration(Some(Duration::from_secs(5)));
                }
            }
        }

        // Show dropped files (if any):
        if !self.files.is_empty() {
            let mut open = true;
            Window::new("Dropped files")
                .open(&mut open)
                .show(ctx, |ui| {
                    for file in &self.files {
                        ui.label(file.info());
                    }
                });
            if !open {
                self.files.clear();
            }
        }
    }

    fn central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if self.files.is_empty() {
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        ui.label("Drag and drop input file here...");
                    },
                );
            } else {
                ui.heading(HEADER);
                ui.separator();
                match self.io {
                    Io::Input => {
                        self.widgets.input.list.input = self.input.take();
                        ui.add(&mut self.widgets.input.list);
                        self.input = self.widgets.input.list.input.take();
                        self.widgets.input.list.expand = None;
                    }
                    Io::Output => match self.widget {
                        Widget::List => {
                            self.widgets.output.list.filter = self.filter.clone();
                            self.widgets.output.list.ui(ui);
                        }
                        Widget::Plot => {
                            self.widgets.output.plot.filter = self.filter.clone();
                            self.widgets.output.plot.ui(ui);
                        }
                        Widget::Table => {
                            self.widgets.output.table.filter = self.filter.clone();
                            self.widgets.output.table.ui(ui);
                        }
                    },
                }
            }
        });
    }

    fn left_panel(&mut self, ctx: &Context) {
        SidePanel::left("left_panel")
            .resizable(false)
            .show_animated(ctx, self.left_panel, |ui| {
                ui.heading("Control Panel");
                ui.separator();
                ui.horizontal(|ui| {
                    ui.selectable_value(&mut self.io, Io::Input, "Input");
                    ui.selectable_value(&mut self.io, Io::Output, "Output");
                });
                ui.separator();
                match self.io {
                    Io::Input => {
                        ui.checkbox(&mut self.widgets.input.list.edit, "Edit");
                        ui.horizontal(|ui| {
                            ui.selectable_value(
                                &mut self.widgets.input.list.expand,
                                Some(false),
                                "Collapse",
                            );
                            ui.selectable_value(
                                &mut self.widgets.input.list.expand,
                                Some(true),
                                "Expand",
                            );
                        });
                    }
                    Io::Output => {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.widget, Widget::List, "List");
                            ui.selectable_value(&mut self.widget, Widget::Table, "Table");
                            ui.selectable_value(&mut self.widget, Widget::Plot, "Plot");
                        });
                    }
                }
                ui.separator();
                ui.horizontal(|ui| {
                    ComboBox::from_label("Positional Composition Kind")
                        .selected_text(PositionalComposition::abbreviation(
                            &self.filter.positional_composition,
                        ))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.filter.positional_composition, None, "");
                            ui.selectable_value(
                                &mut self.filter.positional_composition,
                                Some(PositionalComposition::Specie),
                                "PSC",
                            )
                            .on_hover_text("Positional-Specie Composition");
                            ui.selectable_value(
                                &mut self.filter.positional_composition,
                                Some(PositionalComposition::Type),
                                "PTC",
                            )
                            .on_hover_text("Positional-Type Composition");
                        });
                });
                let text = if self.widgets.output.plot.inverted {
                    "🔄"
                } else {
                    "🔃"
                };
                ui.toggle_value(&mut self.widgets.output.plot.inverted, text);
                // Filter
                ui.separator();
                ui.group(|ui| {
                    ui.heading("Filter");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Key:")
                            .on_hover_text("Key filter by fatty acids in sn positions");
                        for (index, current) in self.filter.pattern.iter_mut().enumerate() {
                            ComboBox::new(format!("fatty_acids_combo_box_{index}"), "")
                                .selected_text(current.as_deref().unwrap_or_default())
                                .show_ui(ui, |ui| {
                                    ui.selectable_value(current, None, "None");
                                    for fatty_acid in &self.available_fatty_acids {
                                        ui.selectable_value(
                                            current,
                                            Some(fatty_acid.clone()),
                                            fatty_acid,
                                        );
                                    }
                                });
                        }
                    });
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Value:")
                            .on_hover_text("Value filter by bound in percent");
                        let value = self.filter.bound.value().copied().unwrap_or_default();
                        ComboBox::new("bound_combo_box", "")
                            .selected_text(self.filter.bound.variant_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.filter.bound,
                                    Bound::Included(value),
                                    "Included",
                                );
                                ui.selectable_value(
                                    &mut self.filter.bound,
                                    Bound::Excluded(value),
                                    "Excluded",
                                );
                                ui.selectable_value(
                                    &mut self.filter.bound,
                                    Bound::Unbounded,
                                    "Unbounded",
                                );
                            })
                            .response
                            .changed();
                        if let Bound::Included(bound) | Bound::Excluded(bound) =
                            &mut self.filter.bound
                        {
                            ui.drag_percent(bound).changed();
                        }
                    });
                });
                // Sort
                ui.group(|ui| {
                    ui.heading("Sort");
                    ui.separator();
                    ComboBox::from_label("")
                        .selected_text(format!("By {:?}", self.filter.sort))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.filter.sort,
                                Some(Sort::Key),
                                Sort::Key.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.filter.sort,
                                Some(Sort::Value),
                                Sort::Value.to_string(),
                            );
                        });
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
    }

    fn top_panel(&mut self, ctx: &Context) {
        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.horizontal_wrapped(|ui| {
                global_dark_light_mode_switch(ui);
                ui.separator();
                ui.toggle_value(&mut self.left_panel, "🛠 Control");
            });
        });
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
        self.top_panel(ctx);
        self.left_panel(ctx);
        self.central_panel(ctx);
        self.file_drag_and_drop_ui(ctx);
        self.toasts.show(ctx);
    }
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct Filter {
    pub bound: Bound<f64>,
    pub pattern: Pattern,
    pub positional_composition: Option<PositionalComposition>,
    pub sort: Option<Sort>,
}

impl Filter {
    pub fn filtered(&self, output: Output) -> Output {
        output
            .bound(self.bound)
            .filter(|composition, _| {
                for tag in composition {
                    if tag == self.pattern {
                        return true;
                    }
                }
                false
            })
            .map(|composition| match self.positional_composition {
                Some(PositionalComposition::Specie) => composition
                    .into_iter()
                    .flat_map(|tag| [tag.clone().reverse(), tag])
                    .collect(),
                _ => composition,
            })
            .sort(self.sort.unwrap_or_default())
    }
}

impl Default for Filter {
    fn default() -> Self {
        Self {
            bound: Bound::Unbounded,
            pattern: default(),
            positional_composition: default(),
            sort: default(),
        }
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Ui {
    text: InputText,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Widgets {
    input: InputWidgets,
    output: OutputWidgets,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct InputWidgets {
    list: InputList,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct OutputWidgets {
    list: OutputList,
    table: OutputTable,
    plot: OutputPlot,
}
