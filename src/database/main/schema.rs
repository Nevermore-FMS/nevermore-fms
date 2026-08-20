// @generated automatically by Diesel CLI.

diesel::table! {
    authentication_tokens (id) {
        id -> Text,
        target_type -> Text,
        target_id -> Text,
        token_hash -> Text,
        created_timestamp -> BigInt,
        expires_at_timestamp -> BigInt,
    }
}

diesel::table! {
    user_credentials (id) {
        id -> Text,
        user_id -> Text,
        credential_type -> Text,
        credential_body -> Text,
        created_timestamp -> BigInt,
    }
}

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        full_name -> Text,
        permissions -> Text,
    }
}

diesel::joinable!(user_credentials -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(authentication_tokens, user_credentials, users,);
