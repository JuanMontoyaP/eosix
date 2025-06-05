use clap::{Parser, Subcommand};

use crate::models::{IdArg, TagArg};

#[derive(Parser)]
#[command(
    name = "Eosix",
    author = "Juan Pablo Montoya",
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
    #[command(
        name = "add",
        about = "Add a new quote",
        long_about = "Add a new quote with the specified text, author, and optional tag."
    )]
    Add {
        #[arg(short, long, value_name = "TEXT", help = "Text of the quote")]
        text: String,

        #[arg(short, long, value_name = "AUTHOR", help = "Author of the quote")]
        author: String,

        #[command(flatten)]
        tags: TagArg,
    },

    #[command(name = "list", about = "List all quotes")]
    List {
        #[command(flatten)]
        tags: TagArg,
    },

    #[command(name = "get", about = "Get a quote")]
    Get {
        #[command(flatten)]
        id: IdArg,
    },

    #[command(name = "update", about = "Update a quote")]
    Update {
        #[command(flatten)]
        id: IdArg,

        #[arg(short, long, value_name = "TEXT", help = "New text for the quote")]
        text: Option<String>,

        #[arg(short, long, value_name = "AUTHOR", help = "New author for the quote")]
        author: Option<String>,

        #[command(flatten)]
        tags: TagArg,
    },

    #[command(name = "delete", about = "Delete a quote")]
    Delete {
        #[command(flatten)]
        id: IdArg,
    },
}
