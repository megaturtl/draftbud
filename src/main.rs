mod commands;
mod utils;

use clap::{Parser, Subcommand};

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Find all recipes for a given item
    Recipe {
        /// Item to get recipes for
        item: String,

        /// Condense required materials, e.g. 8 sticks -> 4 planks -> 1 log
        #[arg(short, long)]
        condense: bool,
    },

    /// List status effects and how to obtain
    #[command(visible_alias = "eff")]
    Effects,

    /// List easy advancements
    #[command(visible_alias = "adv")]
    Advancements,

    /// List easy foods
    Foods,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Recipe { item, condense } => commands::recipe::run(item, condense),
        Command::Effects => commands::effects::run(),
        Command::Advancements => commands::advancements::run(),
        Command::Foods => commands::foods::run(),
    }
}
