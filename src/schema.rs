table! {
    app_user (id) {
        id -> Int4,
        username -> Varchar,
        password_hash -> Varchar,
    }
}

table! {
    bug (id) {
        id -> Int4,
        title -> Varchar,
        body -> Text,
        resolved -> Bool,
    }
}

allow_tables_to_appear_in_same_query!(
    app_user,
    bug,
);
