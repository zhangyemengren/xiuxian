pub mod world;
pub mod app;
pub mod time_line;

use app::App;

fn main() {
    let app = App::new();
    app.run();
}
