use crate::models::{Quote, TagArg};

pub fn handle_add(text: &str, author: &str, tags: &TagArg) {
    // This function will handle the logic for adding a new quote.
    // For now, we will just print a message indicating that the add command was called.

    let quote: Quote = Quote {
        id: 0, // ID will be assigned later
        text: text.to_string(),
        author: author.to_string(),
        tags: tags.clone(),
    };

    let serialize = serde_json::to_string(&quote).unwrap();
    println!("Serialized quote: {}", serialize);
}
