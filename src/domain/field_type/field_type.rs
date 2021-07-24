mod field_type {

    enum FieldTypeCategory {
        Text,
        GoogleImage
    }
    struct FieldType {
        id: i64,
        name: String,
        card_type: CardType,
        category: FieldTypeCategory
    }
}