use diesel::prelude::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::quotes;

#[derive(Queryable, Selectable, Serialize, Deserialize)]
#[diesel(table_name = quotes)]
#[diesel(check_for_backend(diesel::mysql::Mysql))]
pub struct Quote {
    pub id: i32,
    pub body: String,
    pub author: String,
    pub tag: String,
}

#[derive(Insertable)]
#[diesel(table_name = quotes)]
pub struct NewQuote<'a> {
    pub body: &'a str,
    pub author: &'a str,
    pub tag: &'a str,
}
