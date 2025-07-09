use diesel::prelude::*;

use crate::models::{NewQuote, Quote};
use crate::modules::DBConnectionTrait;
use crate::schema::quotes;

pub struct QuoteHandler;

impl DBConnectionTrait for QuoteHandler {}

impl QuoteHandler {
    fn db_connection(&self) -> diesel::MysqlConnection {
        let conn = self.get_connection().expect("Failed to get DB connection");
        conn
    }

    pub fn new() -> Self {
        QuoteHandler
    }

    pub fn create_quote(self, body: &str, author: &str, tag: &str) -> Quote {
        let new_quote = NewQuote { body, author, tag };

        let conn = &mut self.db_connection();

        conn.transaction(|conn| {
            diesel::insert_into(quotes::table)
                .values(&new_quote)
                .execute(conn)?;

            quotes::table
                .order(quotes::id.desc())
                .select(Quote::as_select())
                .first(conn)
        })
        .expect("Error saving new quote")
    }

    pub fn get_quote_by_id(self, id: u32) -> Quote {
        let conn = &mut self.db_connection();

        let quote = quotes::table
            .filter(quotes::id.eq(id as i32))
            .select(Quote::as_select())
            .first(conn)
            .expect(&format!("Error retrieving quote with ID: {}", id));

        quote
    }

    pub fn get_random_quote(self) -> Quote {
        let conn = &mut self.db_connection();

        let random_quote = quotes::table
            .order(diesel::dsl::sql::<diesel::sql_types::Integer>("RAND()"))
            .select(Quote::as_select())
            .first(conn)
            .expect("Error retrieving random quote");

        random_quote
    }

    pub fn list_quote(self) -> Vec<Quote> {
        let conn = &mut self.db_connection();

        let list_quotes = quotes::table
            .order(quotes::id.asc())
            .select(Quote::as_select())
            .load(conn)
            .expect("Error retrieving quotes");

        list_quotes
    }

    pub fn update_quote(self, id: u32, body: &str, author: &str, tag: &str) -> Quote {
        let conn = &mut self.db_connection();

        let updated_quote = conn
            .transaction::<Quote, diesel::result::Error, _>(|conn| {
                diesel::update(quotes::table.filter(quotes::id.eq(id as i32)))
                    .set((
                        quotes::body.eq(body),
                        quotes::author.eq(author),
                        quotes::tag.eq(tag),
                    ))
                    .execute(conn)?;

                let updated_quote = quotes::table
                    .filter(quotes::id.eq(id as i32))
                    .select(Quote::as_select())
                    .first::<Quote>(conn)?;
                Ok(updated_quote)
            })
            .expect(&format!("Error updating quote with ID: {}", id));

        updated_quote
    }

    pub fn delete_quote_by_id(self, id: u32) -> Quote {
        let conn = &mut self.db_connection();

        conn.transaction::<Quote, diesel::result::Error, _>(|conn| {
            let deleted_quote = quotes::table
                .filter(quotes::id.eq(id as i32))
                .select(Quote::as_select())
                .first(conn)?;

            diesel::delete(quotes::table.filter(quotes::id.eq(id as i32))).execute(conn)?;

            Ok(deleted_quote)
        })
        .expect(&format!("Error deleting quote with ID: {}", id))
    }
}
