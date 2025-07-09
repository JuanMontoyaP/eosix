pub mod cli;
pub mod models;
pub mod modules;
pub mod schema;
pub mod storage;

use clap::Parser;

use cli::{Cli, Commands};
use modules::QuoteHandler;

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::AddQuote(quote) => {
            println!("Adding a new quote...");
            let handler = QuoteHandler::new();
            let new_quote = handler.create_quote(&quote.quote_body, &quote.author, &quote.tag);

            println!("{}", serde_json::to_string(&new_quote).unwrap());
        }

        Commands::GetQuote(quote_id) => {
            println!("Retrieving a quote...");
            let handler = QuoteHandler::new();
            let quote = handler.get_quote_by_id(quote_id.id);

            println!("{}", serde_json::to_string(&quote).unwrap());
        }
        Commands::RandomQuote => {
            println!("Retrieving a random quote...");
            let handler = QuoteHandler::new();
            let quote = handler.get_random_quote();

            println!("{}", serde_json::to_string(&quote).unwrap());
        }

        Commands::ListQuotes => {
            println!("Listing all quotes...");

            let handler = QuoteHandler::new();
            let quotes = handler.list_quote();

            println!("{}", serde_json::to_string(&quotes).unwrap());
        }

        Commands::DeleteQuote(quote_id) => {
            println!("Deleting a quote...");
            let handler = QuoteHandler::new();
            let deleted_quote = handler.delete_quote_by_id(quote_id.id);

            println!("{}", serde_json::to_string(&deleted_quote).unwrap());
        }

        Commands::UpdateQuote { id, quote } => {
            println!("Updating a quote...");
            let handler = QuoteHandler::new();
            let updated_quote =
                handler.update_quote(id.id, &quote.quote_body, &quote.author, &quote.tag);

            println!("{}", serde_json::to_string(&updated_quote).unwrap());
        }
    }
}
