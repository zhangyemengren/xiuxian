pub mod app;
pub mod simulation;
pub mod time_line;
pub mod world;

use app::App;

fn main() {
    let app = App::new();
    app.run();
}
