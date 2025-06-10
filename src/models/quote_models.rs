use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::models::cli_models::TagArg;

#[derive(Serialize, Deserialize, Debug)]
pub struct Quote {
    pub id: Uuid,
    pub text: String,
    pub author: String,
    pub tags: TagArg,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct QuoteCollection {
    pub quotes: Vec<Quote>,
}
