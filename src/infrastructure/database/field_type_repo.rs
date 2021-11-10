use shaku::Interface;

#[derive(Default)]
pub struct FieldTypeDO {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub card_type_id: i64,
}

pub trait FieldTypeRepo: Interface {
    fn query_by_id(&self, id: i64) -> FieldTypeDO;
    fn query_by_card_type_id(&self, card_type_id: i64) -> Vec<FieldTypeDO>;
}
