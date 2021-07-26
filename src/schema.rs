table! {
    books (id) {
        id -> Integer,
        name -> Nullable<Integer>,
        contents -> Nullable<Integer>,
        #[sql_name = "type"]
        type_ -> Nullable<Text>,
    }
}

table! {
    card_instances (id) {
        id -> Integer,
        card_id -> Nullable<Integer>,
        composition_id -> Nullable<Integer>,
        due_time -> Nullable<Timestamp>,
        book_id -> Nullable<Integer>,
    }
}

table! {
    card_types (id) {
        id -> Integer,
        name -> Nullable<Text>,
    }
}

table! {
    cards (id) {
        id -> Integer,
        card_type_id -> Nullable<Integer>,
        book_id -> Nullable<Integer>,
        word -> Nullable<Text>,
        create_time -> Nullable<Timestamp>,
    }
}

table! {
    compositions (id) {
        id -> Integer,
        name -> Nullable<Text>,
        card_type_id -> Nullable<Integer>,
        front_type_ids -> Nullable<Text>,
        back_type_ids -> Nullable<Text>,
    }
}

table! {
    configs (id) {
        id -> Integer,
        configs -> Nullable<Text>,
    }
}

table! {
    field_types (id) {
        id -> Integer,
        name -> Nullable<Text>,
        category -> Nullable<Text>,
        card_type_id -> Nullable<Integer>,
    }
}

table! {
    fields (id) {
        id -> Integer,
        original_contents -> Nullable<Text>,
        plain_text_contents -> Nullable<Text>,
        card_id -> Nullable<Integer>,
        field_type_id -> Nullable<Integer>,
    }
}

table! {
    words (id) {
        id -> Integer,
        book_id -> Nullable<Integer>,
        original_word -> Nullable<Text>,
        positions -> Nullable<Text>,
        status -> Nullable<Integer>,
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
