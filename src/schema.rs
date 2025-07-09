// @generated automatically by Diesel CLI.

diesel::table! {
    quotes (id) {
        id -> Integer,
        #[max_length = 255]
        author -> Varchar,
        body -> Text,
        #[max_length = 255]
        tag -> Varchar,
    }
}
