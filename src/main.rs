use anyhow::Result;
use app::App;

mod app;
mod cli;
mod expressions;

fn main() -> Result<()> {
    let app = App::new();
    app.run()
}
