use crate::infrastructure::database::base::BaseQuery;

#[derive(Default)]
pub struct FieldTypeQuery {
    pub base_query: BaseQuery,
    pub id: Option<i64>,
    pub category: Option<String>,
    pub card_type_id: Option<i64>,
}

pub struct FieldTypeDO {
    pub id: i64,
    pub name: String,
    pub category: String,
    pub card_type_id: i64,
}

pub trait FieldTypeRepo {
    fn query_by_id(id: i64) -> FieldTypeDO;
}
