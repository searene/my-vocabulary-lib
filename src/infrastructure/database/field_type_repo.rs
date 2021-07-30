pub enum FieldTypeCategoryInfra {
    Text,
    GoogleImage,
}

pub struct FieldTypeQuery {
    id: i64,
    card_type_id: i64,
}

pub struct FieldTypeDO {
    name: String,
    field_type_category_infra: FieldTypeCategoryInfra,
    card_type_id: i64,
}

pub trait FieldTypeRepo {
    fn query(field_type_query: FieldTypeQuery) -> Vec<FieldTypeDO>;
}
