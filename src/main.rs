//! `tp` / `temprs` binary entry point — delegates to `TempApp::run()`.

use temprs::model::app::TempApp;

fn main() {
    let mut app = TempApp::new();

    app.run();
}
