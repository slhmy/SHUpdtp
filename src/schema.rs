table! {
    users (id) {
        id -> Int4,
        salt -> Nullable<Varchar>,
        hash -> Nullable<Bytea>,
        account -> Text,
        mobile -> Nullable<Text>,
        role -> Text,
    }
}
