use uuid::Uuid;

use crate::models::{Quote, TagArg};
use crate::tools::write_quote_to_file;

pub fn handle_add(text: &str, author: &str, tags: &TagArg) {
    // This function will handle the logic for adding a new quote.
    // For now, we will just print a message indicating that the add command was called.

    let quote: Quote = Quote {
        id: Uuid::new_v4(),
        text: text.to_string(),
        author: author.to_string(),
        tags: tags.clone(),
    };
    write_quote_to_file(&quote);
    println!("Added quote: {:?}", quote);
}
