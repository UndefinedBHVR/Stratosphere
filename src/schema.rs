table! {
    auths (refresh) {
        token -> Varchar,
        refresh -> Varchar,
        owner -> Varchar,
        expiry -> Timestamp,
        created -> Timestamp,
    }
}

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

joinable!(auths -> users (owner));

allow_tables_to_appear_in_same_query!(auths, users,);
