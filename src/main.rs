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

    /// List all suspicious stews
    Stew,
}

fn main() {
    let cli = Cli::parse();
    match cli.command {
        Command::Recipe { item, condense } => commands::recipe::run(item, condense),
        Command::Stew => commands::stew::run(),
    }
}
