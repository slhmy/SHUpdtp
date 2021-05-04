table! {
    access_control_list (region, user_id) {
        user_id -> Int4,
        region -> Text,
    }
}

table! {
    contests (region) {
        region -> Text,
        title -> Text,
        introduction -> Nullable<Text>,
        start_time -> Nullable<Timestamp>,
        end_time -> Nullable<Timestamp>,
        seal_time -> Nullable<Timestamp>,
        settings -> Text,
    }
}

table! {
    problem_sets (region) {
        region -> Text,
        title -> Text,
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
    region_access_settings (region) {
        region -> Text,
        salt -> Nullable<Varchar>,
        hash -> Nullable<Bytea>,
    }
}

table! {
    region_links (inner_id, region) {
        region -> Text,
        inner_id -> Int4,
        problem_id -> Int4,
        score -> Nullable<Int4>,
    }
}

table! {
    regions (name, self_type) {
        name -> Text,
        self_type -> Text,
        title -> Text,
        has_access_setting -> Bool,
        introduction -> Nullable<Text>,
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
        out_results -> Nullable<Array<Text>>,
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
    access_control_list,
    contests,
    problem_sets,
    problems,
    region_access_settings,
    region_links,
    regions,
    samples,
    submissions,
    users,
);
