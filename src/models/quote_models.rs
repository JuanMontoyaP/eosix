use crate::models::cli_models::TagArg;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub id: u64,
    pub text: String,
    pub author: String,
    pub tags: TagArg,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteCollection {
    pub quotes: Vec<Quote>,
}
