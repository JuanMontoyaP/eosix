use clap::Args;

#[derive(Args)]
pub struct QuoteArgs {
    /// The body of the quote.
    #[arg(short, long)]
    pub quote_body: String,

    /// The author of the quote.
    #[arg(short, long)]
    pub author: String,

    /// The tag associated with the quote.
    #[arg(short, long)]
    pub tag: String,
}

#[derive(Args)]
pub struct QuoteIdArgs {
    /// The ID of the quote.
    pub id: u32,
}
