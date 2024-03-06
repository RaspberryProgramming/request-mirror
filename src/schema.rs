// @generated automatically by Diesel CLI.

diesel::table! {
    clients (id) {
        id -> Int8,
        ip -> Text,
        client_id -> Text,
    }
}

diesel::table! {
    history (id) {
        id -> Int8,
        client_id -> Text,
        request_type -> Text,
        timestamp -> Timestamp,
    }
}

diesel::table! {
    ownership (id) {
        id -> Int8,
        owner_id -> Text,
        client_id -> Text,
    }
}

diesel::table! {
    pair_records (id) {
        id -> Int8,
        history_id -> Int8,
        pair_type -> Int2,
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
