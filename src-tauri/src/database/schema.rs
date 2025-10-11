// @generated automatically by Diesel CLI.

diesel::table! {
    bookmark (id) {
        id -> Integer,
        snippet -> Text,
        source_id -> Integer,
        created_at -> Text,
    }
}

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
        source_id -> Nullable<Integer>,
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

diesel::joinable!(bookmark -> source (source_id));
diesel::joinable!(quiz_answer -> source (source_id));

diesel::allow_tables_to_appear_in_same_query!(bookmark, kanji, quiz_answer, source,);
