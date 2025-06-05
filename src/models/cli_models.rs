use clap::Args;
use serde::{Deserialize, Serialize};

#[derive(Args, Serialize, Deserialize, Debug)]
pub struct IdArg {
    #[arg(short, long, value_name = "ID", help = "ID of the quote to get")]
    id: String,
}

#[derive(Args, Serialize, Deserialize, Clone, Debug)]
pub struct TagArg {
    #[arg(long, value_name = "TAG", help = "Optional tags for the quote")]
    tags: Option<Vec<String>>,
}
