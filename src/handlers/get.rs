use crate::models::IdArg;

pub fn handle_get(id: &IdArg) {
    // This function will handle the logic for getting a quote.
    // For now, we will just print a message indicating that the get command was called.
    dbg!("Get command called for ID: {:?}", id);
}
