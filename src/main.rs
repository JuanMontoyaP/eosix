mod cli;
mod handlers;
mod models;

use clap::Parser;
use cli::Cli;
use cli::Commands;

use handlers::{handle_add, handle_delete, handle_get, handle_list, handle_update};

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Add { text, author, tags } => {
            handle_add(text, author, tags);
        }
        Commands::List { tags } => {
            handle_list(tags);
        }
        Commands::Get { id } => {
            handle_get(id);
        }
        Commands::Update {
            id,
            text,
            author,
            tags,
        } => {
            // Handle the update command
            handle_update(id, text, author, tags);
        }
        Commands::Delete { id } => {
            // Handle the delete command
            handle_delete(&id);
        }
    }
}
