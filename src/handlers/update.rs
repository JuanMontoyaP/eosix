use crate::models::{IdArg, TagArg};

pub fn handle_update(
    id: &IdArg,
    new_quote: &Option<String>,
    author: &Option<String>,
    tags: &TagArg,
) {
    // This function will handle the logic for updating a quote.
    // For now, we will just print a message indicating that the update command was called.
    println!("Update command called for ID: {:?}", id);
    if let Some(quote) = new_quote {
        println!("New quote: {}", quote);
    }
    if let Some(author) = author {
        println!("New author: {}", author);
    }
    println!("New tags: {:?}", tags);
    dbg!(tags);
}
