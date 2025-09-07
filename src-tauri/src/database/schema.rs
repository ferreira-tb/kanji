// @generated automatically by Diesel CLI.

diesel::table! {
    kanji (id) {
        id -> Text,
        created_at -> Text,
        updated_at -> Text,
    }
}

diesel::table! {
    quiz_answer (id) {
        id -> Text,
        question -> Text,
        answer -> Text,
        created_at -> Text,
    }
}

diesel::table! {
    source (id) {
        id -> Integer,
        path -> Text,
        name -> Text,
        created_at -> Text,
        updated_at -> Text,
        enabled -> Bool,
        weight -> Integer,
    }
}

diesel::allow_tables_to_appear_in_same_query!(kanji, quiz_answer, source,);
