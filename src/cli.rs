use clap::Parser;

#[derive(Parser)]
#[command(name = "cargo")]
#[command(bin_name = "cargo")]
pub enum Cli {
    FindUnsafe(ExampleDeriveArgs),
}

#[derive(clap::Args)]
#[command(version, about, long_about = None)]
pub struct ExampleDeriveArgs {
    #[arg(long)]
    pub path: Option<std::path::PathBuf>,
}
