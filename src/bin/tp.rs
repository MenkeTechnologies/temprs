//! `tp` — the short name, identical dispatch to `temprs`.
//!
//! A file of its own rather than a second `[[bin]]` on `src/main.rs`: two
//! targets sharing one path make cargo warn on every build and every
//! `cargo doc`.

use temprs::model::app::TempApp;

fn main() {
    let mut app = TempApp::new();

    app.run();
}
