// @generated automatically by Diesel CLI.

diesel::table! {
    users (id) {
        id -> Text,
        username -> Text,
        full_name -> Text,
        permissions -> Text,
    }
}
