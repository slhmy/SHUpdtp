table! {
    problem_sets (set_name, problem_id) {
        set_name -> Text,
        problem_id -> Int4,
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
    regions (name, type_) {
        name -> Text,
        #[sql_name = "type"]
        type_ -> Text,
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

allow_tables_to_appear_in_same_query!(problem_sets, problems, regions, samples, submissions, users,);
