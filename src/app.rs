use crate::{
    config::{Composition, Config, InputView, Io, OutputView, Sort},
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
use egui_notify::Toasts;
use serde::{Deserialize, Serialize};
use std::{default::default, ops::Bound, str, time::Duration};
use toml_edit::Document;
use tracing::error;

// 🔍🔧📝🖹⚙🛠⬇🔃🔄
const HEADER: &str = "Positional-species and positional-type (SUS, SUU, UUU) composition of TAG from mature fruit arils of the Euonymus section species, mol % of total TAG";

fn read(file: &DroppedFile) -> Result<String> {
    let bytes = file
        .bytes
        .as_ref()
        .with_context(|| "Dropped file bytes is none")?;
    let content = String::from_utf8(bytes.to_vec())?;
    Ok(content)
}

fn parse(content: &str) -> Result<Input> {
    let document = content.parse::<Document>()?;
    Ok(Input::new(Visitor::visit(&document)))
}

#[derive(Default, Deserialize, Serialize)]
#[serde(default)]
pub struct App {
    files: Vec<DroppedFile>,
    content: String,
    input: Option<Input>,

    left_panel: bool,
    io: Io,
    output_view: OutputView,
    input_view: InputView,
    available_fatty_acids: Vec<String>,
    config: Config,

    views: Views,
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
            if let Err(err) = try {
                self.content = read(&self.files[0])?;
                let input = parse(&self.content)?;
                self.views.input.list.input = input.clone();
                self.views.output.list.output = input.output();
                self.views.output.plot.output = input.output();
                self.views.output.table.output = input.output();
                self.available_fatty_acids = input.fatty_acids();
                Ok::<_, Error>(())
            } {
                let err: Error = err;
                error!(%err);
                self.toasts
                    .error(format!("{err}"))
                    .set_duration(Some(Duration::from_secs(60)));
            }

            // match parse(&self.files) {
            //     Ok(input) => {
            //         self.available_fatty_acids = input.fatty_acids();
            //         // self.ui.text.text = input.output();
            //         self.ui.output.list.output = input.output();
            //         self.ui.output.plot.output = input.output();
            //         self.ui.output.table.output = input.output();
            //         self.ui.input.list.input = input.clone();
            //         self.input = Some(input);
            //     }
            //     Err(err) => {
            //         error!(%err);
            //     }
            // }
        }

        // Show dropped files (if any):
        if !self.files.is_empty() {
            let mut open = true;
            Window::new("Dropped files")
                .anchor(Align2::RIGHT_BOTTOM, [0.0, 0.0])
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

    // fn set_content(&mut self, content: String) {
    //     self.content = content;
    //     let input = parse(&self.content)?;
    //     self.views.input.list.input = input.clone();
    //     self.views.output.list.output = input.output();
    //     self.views.output.plot.output = input.output();
    //     self.views.output.table.output = input.output();
    //     self.available_fatty_acids = input.fatty_acids();
    // }

    fn bottom_panel(&mut self, ctx: &Context) {}

    fn central_panel(&mut self, ctx: &Context) {
        CentralPanel::default().show(ctx, |ui| {
            if self.files.is_empty() {
                ui.with_layout(
                    Layout::centered_and_justified(Direction::LeftToRight),
                    |ui| {
                        ui.label("Drag and drop input file here...");
                    },
                );
                return;
            }
            match self.io {
                Io::Input => match self.input_view {
                    InputView::List => {
                        self.views.input.list.ui(ui);
                        let input = &self.views.input.list.input;
                        self.views.output.list.output = input.output();
                        self.views.output.plot.output = input.output();
                        self.views.output.table.output = input.output();
                    }
                    InputView::Text => {
                        self.views.input.text.text = self.content.clone();
                        self.views.input.text.ui(ui);
                        self.content = self.views.input.text.text.clone();
                        let input = parse(&self.content).unwrap();
                        self.views.input.list.input = input.clone();
                        self.views.output.list.output = input.output();
                        self.views.output.plot.output = input.output();
                        self.views.output.table.output = input.output();
                        self.available_fatty_acids = input.fatty_acids();
                    }
                },
                Io::Output => match self.output_view {
                    OutputView::List => {
                        self.views.output.list.config = self.config.clone();
                        self.views.output.list.ui(ui);
                    }
                    OutputView::Plot => {
                        self.views.output.plot.config = self.config.clone();
                        self.views.output.plot.ui(ui);
                    }
                    OutputView::Table => {
                        self.views.output.table.config = self.config.clone();
                        self.views.output.table.ui(ui);
                    }
                },
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
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.input_view, InputView::List, "List");
                            ui.selectable_value(&mut self.input_view, InputView::Text, "Text");
                        });
                        if let InputView::List = self.input_view {
                            ui.checkbox(&mut self.views.input.list.edit, "Edit");
                            ui.horizontal(|ui| {
                                ui.selectable_value(
                                    &mut self.views.input.list.open,
                                    Some(false),
                                    "Collapse",
                                );
                                ui.selectable_value(
                                    &mut self.views.input.list.open,
                                    Some(true),
                                    "Expand",
                                );
                            });
                        }
                    }
                    Io::Output => {
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.output_view, OutputView::List, "List");
                            ui.selectable_value(&mut self.output_view, OutputView::Table, "Table");
                            ui.selectable_value(&mut self.output_view, OutputView::Plot, "Plot");
                        });
                    }
                }
                ui.separator();
                ui.horizontal(|ui| {
                    ComboBox::from_label("Composition")
                        .selected_text(Composition::abbreviation(&self.config.composition))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(&mut self.config.composition, None, "");
                            ui.selectable_value(
                                &mut self.config.composition,
                                Some(Composition::PositionalSpecie),
                                "Positional-Specie",
                            );
                            ui.selectable_value(
                                &mut self.config.composition,
                                Some(Composition::PositionalType),
                                "Positional-Type",
                            );
                        });
                });
                ui.toggle_value(&mut self.views.output.plot.stacked, "☰")
                    .on_hover_text("Stacked");
                // Filter
                ui.separator();
                ui.group(|ui| {
                    ui.heading("Filter");
                    ui.separator();
                    ui.horizontal(|ui| {
                        ui.label("Key:")
                            .on_hover_text("Key filter by fatty acids in sn positions");
                        for (index, current) in self.config.pattern.iter_mut().enumerate() {
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
                        let value = self.config.bound.value().copied().unwrap_or_default();
                        ComboBox::new("bound_combo_box", "")
                            .selected_text(self.config.bound.variant_name())
                            .show_ui(ui, |ui| {
                                ui.selectable_value(
                                    &mut self.config.bound,
                                    Bound::Included(value),
                                    "Included",
                                );
                                ui.selectable_value(
                                    &mut self.config.bound,
                                    Bound::Excluded(value),
                                    "Excluded",
                                );
                                ui.selectable_value(
                                    &mut self.config.bound,
                                    Bound::Unbounded,
                                    "Unbounded",
                                );
                            })
                            .response
                            .changed();
                        if let Bound::Included(bound) | Bound::Excluded(bound) =
                            &mut self.config.bound
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
                        .selected_text(format!("By {:?}", self.config.sort))
                        .show_ui(ui, |ui| {
                            ui.selectable_value(
                                &mut self.config.sort,
                                Some(Sort::Key),
                                Sort::Key.to_string(),
                            );
                            ui.selectable_value(
                                &mut self.config.sort,
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

    fn windows(&mut self, ctx: &Context) {
        // self.views.input.list.window(ctx);
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
        // self.bottom_panel(ctx);
        self.windows(ctx);
        self.file_drag_and_drop_ui(ctx);
        // self.toasts.show(ctx);
    }
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct Views {
    input: InputViews,
    output: OutputViews,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct InputViews {
    list: InputList,
    text: InputText,
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
struct OutputViews {
    list: OutputList,
    table: OutputTable,
    plot: OutputPlot,
}
