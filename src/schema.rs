table! {
    users (id) {
        id -> Int4,
        salt -> Nullable<Varchar>,
        hash -> Nullable<Bytea>,
        name -> Text,
        mobile -> Nullable<Text>,
        role -> Text,
    }
}
