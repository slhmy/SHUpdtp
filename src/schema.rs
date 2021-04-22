table! {
    problem_sets (region) {
        region -> Text,
        name -> Text,
        introduction -> Nullable<Text>,
    }
}

table! {
    problems (id) {
        id -> Int4,
        title -> Text,
        tags -> Array<Text>,
        difficulty -> Float8,
        contents -> Text,
        settings -> Text,
        is_released -> Bool,
    }
}

table! {
    region_links (region) {
        region -> Text,
        problem_id -> Int4,
        score -> Nullable<Int4>,
    }
}

table! {
    regions (name, self_type) {
        name -> Text,
        self_type -> Text,
    }
}

table! {
    samples (submission_id) {
        submission_id -> Uuid,
        description -> Nullable<Text>,
    }
}

table! {
    submissions (id) {
        id -> Uuid,
        problem_id -> Int4,
        user_id -> Int4,
        region -> Nullable<Text>,
        state -> Text,
        settings -> Text,
        result -> Nullable<Text>,
        submit_time -> Timestamp,
        is_accepted -> Nullable<Bool>,
        finish_time -> Nullable<Timestamp>,
        max_time -> Nullable<Int4>,
        max_memory -> Nullable<Int4>,
        language -> Nullable<Text>,
        err -> Nullable<Text>,
    }
}

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

allow_tables_to_appear_in_same_query!(
    problem_sets,
    problems,
    region_links,
    regions,
    samples,
    submissions,
    users,
);
