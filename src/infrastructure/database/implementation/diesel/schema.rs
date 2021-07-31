table! {
    books (id) {
        id -> BigInt,
        name -> Nullable<BigInt>,
        contents -> Nullable<BigInt>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
    }
}

table! {
    card_instances (id) {
        id -> BigInt,
        card_id -> Nullable<BigInt>,
        composition_id -> Nullable<BigInt>,
        due_time -> Nullable<Timestamp>,
        book_id -> Nullable<BigInt>,
    }
}

table! {
    card_types (id) {
        id -> BigInt,
        name -> Nullable<Text>,
    }
}

table! {
    cards (id) {
        id -> BigInt,
        card_type_id -> Nullable<BigInt>,
        book_id -> Nullable<BigInt>,
        word -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    compositions (id) {
        id -> BigInt,
        name -> Nullable<Text>,
        card_type_id -> Nullable<BigInt>,
        front_type_ids -> Nullable<Text>,
        back_type_ids -> Nullable<Text>,
    }
}

table! {
    configs (id) {
        id -> BigInt,
        #[sql_name = "type"]
        configs_ -> Nullable<Text>,
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
        card_id -> Nullable<BigInt>,
        field_type_id -> Nullable<BigInt>,
    }
}

table! {
    words (id) {
        id -> BigInt,
        book_id -> Nullable<BigInt>,
        original_word -> Nullable<Text>,
        positions -> Nullable<Text>,
        status -> Nullable<BigInt>,
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
