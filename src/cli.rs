use clap::Parser;

#[derive(Debug, Clone, Parser)]
pub struct CliArgs {
    pub expression: String,
}
