use anyhow::Result;
use app::App;

mod app;
mod cli;

fn main() -> Result<()> {
    let app = App::new();
    app.run()
}
