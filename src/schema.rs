table! {
    users (id) {
        id -> Varchar,
        nickname -> Varchar,
        email -> Varchar,
        password -> Varchar,
        rank -> Int4,
        is_priv -> Bool,
        updated_at -> Timestamp,
        created_at -> Timestamp,
    }
}
