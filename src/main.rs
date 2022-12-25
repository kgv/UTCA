#![feature(default_free_fn)]
#![feature(try_blocks)]
#![feature(try_trait_v2)]
#![feature(type_name_of_val)]
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use app::App;
use config::Config;
use input::Input;
use output::Output;
use specie::Specie;
use tag::{Tag, Tags};
use visitor::Visitor;

// When compiling natively
#[cfg(not(target_arch = "wasm32"))]
fn main() {
    // Log to stdout (if you run with `RUST_LOG=debug`).
    tracing_subscriber::fmt::init();

    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "eframe template",
        native_options,
        Box::new(|cc| Box::new(App::new(cc))),
    );
}

// When compiling to web using trunk
#[cfg(target_arch = "wasm32")]
fn main() {
    // Make sure panics are logged using `console.error`.
    console_error_panic_hook::set_once();

    // Redirect tracing to console.log and friends:
    tracing_wasm::set_as_global_default();

    let web_options = eframe::WebOptions::default();
    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "the_canvas_id", // hardcode it
            web_options,
            Box::new(|cc| Box::new(App::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}

mod app;
mod config;
mod dataset;
mod input;
mod output;
mod specie;
mod tag;
mod utils;
mod visitor;
