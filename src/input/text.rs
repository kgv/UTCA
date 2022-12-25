use std::{default::default, ops::Range};

use egui::{
    text::{LayoutJob, LayoutSection, TextFormat},
    util::cache::{ComputerMut, FrameCache},
    Color32, Context, FontId, ScrollArea, Stroke, TextEdit, TextStyle, Ui,
};
use serde::{Deserialize, Serialize};
use syntect::{
    easy::HighlightLines,
    highlighting::{FontStyle, ThemeSet},
    parsing::SyntaxSet,
    util::LinesWithEndings,
};

// use linfa::traits::Transformer;
// use linfa_hierarchical::HierarchicalCluster;
// use linfa_kernel::{Kernel, KernelMethod};

// let dataset = linfa_datasets::iris();
// let kernel = Kernel::params()
//     .method(KernelMethod::Gaussian(1.0))
//     .transform(dataset.records().view());
// let kernel = HierarchicalCluster::default()
//     .num_clusters(3)
//     .transform(kernel)
//     .unwrap();
// kernel.targets();

/// Memoized Code highlighting
pub fn highlight(ctx: &Context, theme: &str, code: &str, language: &str) -> LayoutJob {
    let mut memory = ctx.memory();
    let highlight_cache = memory.caches.cache::<FrameCache<LayoutJob, Highlighter>>();
    highlight_cache.get((theme, code, language))
}

#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct Text {
    pub text: String,
}

// Self::Base16EightiesDark => "base16-eighties.dark",
// Self::Base16MochaDark => "base16-mocha.dark",
// Self::Base16OceanDark => "base16-ocean.dark",
// Self::Base16OceanLight => "base16-ocean.light",
// Self::InspiredGitHub => "InspiredGitHub",
// Self::SolarizedDark => "Solarized (dark)",
// Self::SolarizedLight => "Solarized (light)",
impl Text {
    pub fn ui(&mut self, ui: &mut Ui) {
        // let theme = if ctx.style().visuals.dark_mode {
        //     "base16-mocha.dark"
        //     SyntectTheme::Base16MochaDark
        // } else {
        //     "Solarized (light)"
        //     SyntectTheme::SolarizedLight
        // };
        let theme = "Solarized (dark)";
        let mut layouter = |ui: &Ui, string: &str, wrap_width: f32| {
            let mut layout_job = highlight(ui.ctx(), theme, string, "rs");
            layout_job.wrap.max_width = wrap_width;
            ui.fonts().layout_job(layout_job)
        };
        ui.centered_and_justified(|ui| {
            ScrollArea::both().auto_shrink([false; 2]).show(ui, |ui| {
                ui.add(
                    TextEdit::multiline(&mut self.text)
                        .font(TextStyle::Monospace)
                        .code_editor()
                        .desired_rows(10)
                        .lock_focus(true)
                        .desired_width(f32::INFINITY)
                        .layouter(&mut layouter),
                );
            });
        });
    }
}

/// Highlighter
// #[derive(Debug, Default)]
#[derive(Debug)]
struct Highlighter {
    ps: SyntaxSet,
    ts: ThemeSet,
}

impl Default for Highlighter {
    fn default() -> Self {
        Self {
            ps: SyntaxSet::load_defaults_newlines(),
            ts: ThemeSet::load_defaults(),
        }
    }
}

impl Highlighter {
    pub fn try_highlight(&self, theme: &str, text: &str, language: &str) -> Option<LayoutJob> {
        let syntax = self
            .ps
            .find_syntax_by_name(language)
            .or_else(|| self.ps.find_syntax_by_extension(language))?;
        let mut lines = HighlightLines::new(syntax, &self.ts.themes[theme]);
        let mut job = LayoutJob {
            text: text.to_string(),
            ..default()
        };
        for line in LinesWithEndings::from(text) {
            for (style, range) in lines.highlight_line(line, &self.ps).ok()? {
                let fg = style.foreground;
                let text_color = Color32::from_rgb(fg.r, fg.g, fg.b);
                let italics = style.font_style.contains(FontStyle::ITALIC);
                let underline = style.font_style.contains(FontStyle::ITALIC);
                let underline = if underline {
                    Stroke::new(1.0, text_color)
                } else {
                    Stroke::NONE
                };
                let byte_range = {
                    let text_start = text.as_ptr() as usize;
                    let range_start = range.as_ptr() as usize;
                    assert!(text_start <= range_start);
                    assert!(range_start + range.len() <= text_start + text.len());
                    let offset = range_start - text_start;
                    offset..(offset + range.len())
                };
                job.sections.push(LayoutSection {
                    leading_space: 0.0,
                    byte_range,
                    format: TextFormat {
                        font_id: FontId::monospace(12.0),
                        color: text_color,
                        italics,
                        underline,
                        ..default()
                    },
                });
            }
        }
        Some(job)
    }

    pub fn highlight(&self, theme: &str, text: &str, language: &str) -> LayoutJob {
        self.try_highlight(theme, text, language)
            .unwrap_or_else(|| {
                // Fallback:
                LayoutJob::simple(
                    text.into(),
                    FontId::monospace(12.0),
                    // if theme.dark_mode {
                    //     Color32::LIGHT_GRAY
                    // } else {
                    //     Color32::DARK_GRAY
                    // },
                    Color32::DARK_GRAY,
                    f32::INFINITY,
                )
            })
    }
}

impl ComputerMut<(&str, &str, &str), LayoutJob> for Highlighter {
    fn compute(&mut self, (theme, text, language): (&str, &str, &str)) -> LayoutJob {
        self.highlight(theme, text, language)
    }
}
