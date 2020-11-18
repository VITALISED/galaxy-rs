table! {
    users (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        pass_hash -> Varchar,
        avatar -> Nullable<Text>,
        bio -> Nullable<Varchar>,
        big_bio -> Nullable<Text>,
        created_at -> Timestamp,
    }
}

table! {
    confirmations (id) {
        id -> Uuid,
        username -> Varchar,
        email -> Varchar,
        expires_at -> Timestamp,
    }
}
