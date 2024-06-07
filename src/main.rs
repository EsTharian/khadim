mod check;
mod enums;
mod subcommands;

use clap::Parser;
use enums::subcommands::Main as Subcommands;
use subcommands::init::main as init;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    #[command(subcommand)]
    command: Option<Subcommands>,
}

fn main() {
    check::requirements();

    let cli = Cli::parse();

    match &cli.command {
        Some(Subcommands::Init { name, path }) => init(name.clone(), path.clone()),

        None => (),
    }
}
