// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int4,
        ip -> Text,
        mirror_id -> Text,
    }
}

diesel::table! {
    history (id) {
        id -> Int4,
        client_id -> Text,
        request_type -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    ownership (id) {
        id -> Int4,
        owner_id -> Text,
        client_id -> Text,
    }
}

diesel::table! {
    pair_records (id) {
        id -> Int4,
        history_id -> Int4,
        pair_type -> Int4,
        key -> Text,
        value -> Text,
    }
}

diesel::allow_tables_to_appear_in_same_query!(
    clients,
    history,
    ownership,
    pair_records,
);
