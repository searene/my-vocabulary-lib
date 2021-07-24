mod field_type {

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
        fn query_by_id(field_type_query: FieldTypeQuery) -> Vec<FieldTypeDO>;
    }

    mod implementation {
        mod diesel {
            use crate::infrastructure::table::field_type::field_type::{FieldTypeRepo, FieldTypeDO};

            #[derive(Queryable)]
            struct FieldType {
                pub id: i64,
                pub name: String,
                pub category: String,
                pub card_type_id: i64,
            }

            struct DieselFieldTypeRepo;

            impl FieldTypeRepo for DieselFieldTypeRepo {
                fn query_by_id(field_type_id: i64) -> Vec<FieldTypeDO> {
                    todo!()
                }
            }
        }
    }
}
