use shaku::Interface;

#[derive(Default)]
pub struct FieldTypeDO {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub card_type_id: i64,
}

pub trait FieldTypeRepo: Interface {
    fn query_by_id(id: i64) -> FieldTypeDO where Self: Sized;
    fn query_by_card_type_id(card_type_id: i64) -> Vec<FieldTypeDO> where Self: Sized;
}
