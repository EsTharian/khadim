use clap::Subcommand;

#[derive(Subcommand)]
pub enum Main {
    #[command(about = "Initialize a new Clove.rs project")]
    Init {
        /// The name of the project
        #[arg(short, long, default_value = "clove")]
        name: Option<String>,

        /// The path to the project
        #[arg(short, long, default_value = ".")]
        path: Option<String>,
    },
}
