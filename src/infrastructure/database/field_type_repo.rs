#[derive(Default)]
pub struct FieldTypeDO {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub card_type_id: i64,
}

pub trait FieldTypeRepo {
    fn query_by_id(id: i64) -> FieldTypeDO;
    fn query_by_card_type_id(card_type_id: i64) -> Vec<FieldTypeDO>;
}
