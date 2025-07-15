use clap::Parser;

#[derive(Parser)]
pub struct CliArgs {
    pub expression: String,
    #[arg(short, long = "show-sum")]
    pub show_sum: bool,
}
