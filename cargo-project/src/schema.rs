table! {
    github_user_records (id) {
        id -> BigInt,
        user_id -> Integer,
        login -> Text,
        avatar_url -> Text,
        html_url -> Text,
    }
}

table! {
    permissions (id) {
        id -> Integer,
        user_id -> Integer,
        name -> Text,
    }
}

table! {
    snippets (id) {
        id -> Integer,
        creator_id -> Integer,
        taxonomy -> Text,
        hidden -> Bool,
        title -> Text,
        icon -> Text,
        shared_by -> Text,
        shared_on -> Timestamp,
        summary -> Text,
        description -> Text,
        href -> Text,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}

table! {
    users (id) {
        id -> Integer,
        preferred_name -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    github_user_records,
    permissions,
    snippets,
    users,
);
