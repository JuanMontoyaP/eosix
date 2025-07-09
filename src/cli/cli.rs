use crate::cli::args::{QuoteArgs, QuoteIdArgs};

use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "Eosix",
    author = "Juan Pablo M",
    about,
    version,
    long_about = None
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Add a new quote, requires quote body, author, and tag.
    AddQuote(QuoteArgs),

    /// Get a quote by its ID.
    GetQuote(QuoteIdArgs),

    /// Get a random quote.
    RandomQuote,

    /// List all quotes.
    ListQuotes,

    /// Delete a quote by its ID.
    DeleteQuote(QuoteIdArgs),

    /// Update a quote by its ID.
    UpdateQuote {
        #[command(flatten)]
        id: QuoteIdArgs,
        #[command(flatten)]
        quote: QuoteArgs,
    },
}
