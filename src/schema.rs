// @generated automatically by Diesel CLI.

diesel::table! {
    analytics (id) {
        id -> Int4,
        long_url -> Varchar,
        long_url_id -> Nullable<Int4>,
        created_on -> Timestamp,
    }
}

diesel::table! {
    url (id) {
        id -> Int4,
        short_url -> Varchar,
        long_url -> Varchar,
        created_on -> Timestamp,
    }
}

diesel::joinable!(analytics -> url (long_url_id));

diesel::allow_tables_to_appear_in_same_query!(
    analytics,
    url,
);
