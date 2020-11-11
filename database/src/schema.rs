table! {
    users (id) {
        id -> Int4,
        username -> Varchar,
        pass_hash -> Varchar,
        avatar -> Nullable<Text>,
        bio -> Nullable<Varchar>,
        big_bio -> Nullable<Text>,
    }
}
