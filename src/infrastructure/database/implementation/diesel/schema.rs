table! {
    books (id) {
        id -> BigInt,
        name -> BigInt,
        contents -> BigInt,
        #[sql_name = "type"]
        type_ -> Text,
    }
}

table! {
    card_instances (id) {
        id -> BigInt,
        card_id -> BigInt,
        composition_id -> BigInt,
        due_time -> Timestamp,
        book_id -> BigInt,
    }
}

table! {
    card_types (id) {
        id -> BigInt,
        name -> Text,
    }
}

table! {
    cards (id) {
        id -> BigInt,
        card_type_id -> BigInt,
        book_id -> BigInt,
        word -> Text,
        create_time -> Timestamp,
    }
}

table! {
    compositions (id) {
        id -> BigInt,
        name -> Text,
        card_type_id -> BigInt,
        front_type_ids -> Text,
        back_type_ids -> Text,
    }
}

table! {
    configs (id) {
        id -> BigInt,
        #[sql_name = "type"]
        configs_ -> Text,
    }
}

table! {
    field_types (id) {
        id -> BigInt,
        name -> Text,
        category -> Text,
        card_type_id -> BigInt,
    }
}

table! {
    fields (id) {
        id -> BigInt,
        original_contents -> Nullable<Text>,
        plain_text_contents -> Nullable<Text>,
        card_id -> BigInt,
        field_type_id -> BigInt,
    }
}

table! {
    words (id) {
        id -> BigInt,
        book_id -> BigInt,
        original_word -> Text,
        positions -> Text,
        status -> BigInt,
    }
}

allow_tables_to_appear_in_same_query!(
    books,
    card_instances,
    card_types,
    cards,
    compositions,
    configs,
    field_types,
    fields,
    words,
);
