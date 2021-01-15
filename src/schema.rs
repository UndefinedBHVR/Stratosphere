table! {
    users (id) {
        id -> Varchar,
        nickname -> Nullable<Varchar>,
        email -> Varchar,
        password -> Varchar,
        rank -> Int4,
        is_priv -> Bool,
        updated_at -> Nullable<Timestamp>,
        created_at -> Nullable<Timestamp>,
    }
}
