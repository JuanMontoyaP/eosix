use std::io::Write;

use crate::models::Quote;

use crate::db::open_file;

fn serialize_quote(quote: &Quote) -> String {
    // This function will handle the serialization of a quote.
    // For now, we will just return the quote as a string.
    serde_json::to_string(quote)
        .unwrap_or_else(|_| panic!("Failed to serialize quote: {}", quote.id))
}

pub fn write_quote_to_file(quote: &Quote) {
    // This function will handle the logic for writing a quote to a file.
    // For now, we will just print a message indicating that the write command was called.
    let serialized = serialize_quote(quote);

    let mut file = open_file(true);

    file.write(serialized.as_bytes())
        .expect("Failed to write to file");
    file.write(b"\n") // Add a newline after each quote for better readability
        .expect("Failed to write newline to file");
}
