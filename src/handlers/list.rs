use crate::models::TagArg;

pub fn handle_list(tags: &TagArg) {
    // This function will handle the logic for listing all quotes.
    // For now, we will just print a message indicating that the list command was called.
    println!("List command called");
    dbg!(tags);
}
