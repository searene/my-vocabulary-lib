mod card_facade {

    pub struct FieldTypeVO {

        // field type id
        id: i64,

        // field type category
        category: String,

        // field type name
        name: String,
    }

    pub struct SaveCardParamVO {
        word: String,
        book_id: i64,
        card_type_id: i64,
        field_contents: Vec<FieldContentVO>
    }

    pub struct FieldContentVO {
        field_type_id: i64,
        original_contents: str,
        plain_text_contents: str,
    }

    fn get_field_types(card_type_id: i64) -> Vec<FieldTypeVO> {
        todo!()
    }
}