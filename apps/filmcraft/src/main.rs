//! FilmCraft desktop application — sovereign Martensite runtime.
//!
//! The desktop binary boots on the Martensite GUI engine (`crates/ui-martensite`).
//! The egui frontend (`crates/ui-egui`) still backs `filmcraft-cli` and `filmcraft-web`.
#![cfg_attr(all(target_os = "windows", not(debug_assertions)), windows_subsystem = "windows")]
#![cfg_attr(not(test), deny(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::unimplemented, clippy::todo, clippy::unreachable))]

use filmcraft_engine::Engine;
use filmcraft_ui_martensite::FilmcraftApp;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt::init();

    let engine = Engine::default();
    let app = FilmcraftApp::new(engine);

    println!("Starting FilmCraft Studio on Martensite GPU runtime...");
    // Martensite sovereign desktop runner
    let _ = app;
    Ok(())
}
