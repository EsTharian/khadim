use clap::{Parser, Subcommand};
mod check;

#[derive(Parser)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    #[command(about = "Initialize a new Clove.rs project")]
    Init {
        #[arg(short, long)]
        force: bool,
    },
}

fn main() {
    check::requirements();

    let cli = Cli::parse();
}
