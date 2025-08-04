use anyhow::Result;
use app::App;

mod app;
mod cli;

fn main() -> Result<()> {
    pretty_env_logger::init();

    let app = App::new();
    app.run()
}
