use model::app::TempApp;

mod model;
mod util;

fn main() {
    let mut app = TempApp::new();

    app.run();
}
