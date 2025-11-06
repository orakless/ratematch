// @generated automatically by Diesel CLI.

diesel::table! {
    event (id) {
        id -> Int4,
        #[max_length = 128]
        name -> Varchar,
        #[max_length = 64]
        promotion -> Varchar,
        date -> Date,
    }
}

diesel::table! {
    language (code) {
        #[max_length = 3]
        code -> Bpchar,
    }
}

diesel::table! {
    #[sql_name = "match"]
    match_ (id) {
        id -> Int4,
        event_id -> Int4,
        workers -> Text,
    }
}

diesel::table! {
    match_desc (id) {
        id -> Int4,
        match_id -> Int4,
        description -> Text,
        #[max_length = 3]
        language_code -> Bpchar,
    }
}

diesel::table! {
    rating (id) {
        id -> Int4,
        match_id -> Int4,
        #[max_length = 3]
        language_code -> Bpchar,
        #[max_length = 32]
        username -> Varchar,
        score -> Numeric,
        opinion -> Nullable<Text>,
    }
}

diesel::joinable!(match_ -> event (event_id));
diesel::joinable!(match_desc -> language (language_code));
diesel::joinable!(match_desc -> match_ (match_id));
diesel::joinable!(rating -> language (language_code));
diesel::joinable!(rating -> match_ (match_id));

diesel::allow_tables_to_appear_in_same_query!(event, language, match_, match_desc, rating,);
