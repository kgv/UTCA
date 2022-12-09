use crate::{
    config::{Io, PositionalComposition, Sort, Widget},
    input::{ListWidget as InputListWidget, Text},
    output::{
        ListWidget as OutputListWidget, PlotWidget as OutputPlotWidget,
        TableWidget as OutputTableWidget,
    },
    utils::{BoundExt, Info, UiExt},
    Input, Visitor,
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
use serde::{Deserialize, Serialize};
use std::{default::default, ops::Bound, str, time::Duration};
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

#[derive(Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    files: Vec<DroppedFile>,
    input: Option<Input>,
    io: Io,
    widget: Widget,
    inverted: bool,
    sort: Sort,
    available_fatty_acids: Vec<String>,
    current_fatty_acids: [String; 3],
    bound: Bound<f64>,
    changed: bool,
    left_panel: bool,
    info: bool,
    positional_composition: Option<PositionalComposition>,

    #[serde(skip)]
    img_offset: Pos2,
    #[serde(skip)]
    available_height: f32,

    ui: Ui,
    widgets: Widgets,
    #[serde(skip)]
    toasts: Toasts,
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
            left_panel: true,
            info: false,
            positional_composition: default(),

            img_offset: default(),
            available_height: default(),

            ui: default(),
            toasts: default(),
            widgets: default(),
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

        // Save dropped fileserror!("ERROR dropped files");

        if !ctx.input().raw.dropped_files.is_empty() && self.files.is_empty() {
            self.files = ctx.input().raw.dropped_files.clone();
            match parse_input(&self.files) {
                Ok(input) => {
                    self.available_fatty_acids = input.fatty_acids();
                    self.input = Some(input);
                }
                Err(err) => error!(%err),
            }
        }
    }

    fn central_panel(&mut self, ctx: &Context) {
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
            } else {
                ui.heading(HEADER);
                ui.separator();
                match self.io {
                    Io::Input => {
                        self.widgets.input.list.data = self.input.take();
                        ui.add(&mut self.widgets.input.list);
                        self.input = self.widgets.input.list.data.take();
                        self.widgets.input.list.expand = None;
                    }
                    Io::Output => {
                        match self.widget {
                            Widget::List => {
                                self.widgets.output.list.bound = self.bound;
                                self.widgets.output.list.fatty_acids =
                                    self.current_fatty_acids.clone();
                                self.widgets.output.list.sort = self.sort;
                                self.widgets.output.list.ui(ui);
                                self.bound = self.widgets.output.list.bound;
                                self.sort = self.widgets.output.list.sort;
                            }
                            Widget::Plot => {
                                // ui.add(&mut output.plot(self.inverted));
                            }
                            Widget::Table => {
                                self.widgets.output.table.bound = self.bound;
                                self.widgets.output.table.fatty_acids =
                                    self.current_fatty_acids.clone();
                                self.widgets.output.table.positional_composition =
                                    self.positional_composition;
                                self.widgets.output.table.ui(ui);
                                self.bound = self.widgets.output.table.bound;
                                self.current_fatty_acids =
                                    self.widgets.output.table.fatty_acids.clone();
                                self.positional_composition =
                                    self.widgets.output.table.positional_composition;
                            }
                        }
                    }
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
                PositionalComposition::name(&self.positional_composition);
                ui.separator();
                ui.horizontal(|ui| {
                    ui.combo_box("positional_composition")
                        .selected_text(PositionalComposition::abbreviation(
                            &self.positional_composition,
                        ))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.positional_composition, None, "");
                            ui.selectable_value(
                                &mut self.positional_composition,
                                Some(PositionalComposition::Specie),
                                "PSC",
                            )
                            .on_hover_text("Positional-Specie Composition");
                            ui.selectable_value(
                                &mut self.positional_composition,
                                Some(PositionalComposition::Type),
                                "PTC",
                            )
                            .on_hover_text("Positional-Type Composition");
                        });
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
                                        ui.selectable_value(
                                            current,
                                            fatty_acid.clone(),
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
                            self.changed |= ui.drag_percent(bound).changed();
                        }
                    });
                });
                // Sort
                ui.group(|ui| {
                    ui.heading("Sort");
                    ui.separator();
                    ComboBox::from_label("")
                        .selected_text(format!("By {}", self.sort))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.sort, Sort::Key, Sort::Key.to_string());
                            ui.selectable_value(
                                &mut self.sort,
                                Sort::Value,
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
                        if ui.button("File").clicked() {
                            Window::new("title").open(&mut self.info).show(ctx, |ui| {
                                ui.heading("text");
                                ctx.settings_ui(ui);
                            });
                            // let mut dialog = FileDialog::open_file(None);
                            // dialog.open();
                            // #[cfg(target_arch = "wasm32")]
                            // wasm_bindgen_futures::spawn_local(async {
                            // });
                        }
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
        self.widgets.output.list.windows(ctx);
        self.file_drag_and_drop_ui(ctx);
        self.toasts.show(ctx);
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Ui {
    text: Text,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Widgets {
    input: InputWidgets,
    output: OutputWidgets,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct InputWidgets {
    list: InputListWidget,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct OutputWidgets {
    list: OutputListWidget,
    table: OutputTableWidget,
    plot: OutputPlotWidget,
}
