mod field_type {
    use crate::infrastructure::database::field_type_repo::{FieldTypeDO, FieldTypeRepo};
    use diesel::{Queryable, QueryDsl, RunQueryDsl};
    use crate::infrastructure::database::implementation::diesel::connection::{CONNECTION};
    use crate::infrastructure::database::implementation::diesel::schema::field_types::dsl;
    use crate::infrastructure::database::implementation::diesel::schema::field_types;
    use crate::diesel::ExpressionMethods;

    #[derive(Queryable)]
    struct FieldType {
        pub id: i64,
        pub name: String,
        pub category: String,
        pub card_type_id: i64,
    }

    struct DieselFieldTypeRepo;

    impl FieldTypeRepo for DieselFieldTypeRepo {
        fn query_by_id(id: i64) -> FieldTypeDO {
            let field_type = dsl::field_types.find(id).first(&*CONNECTION.lock().unwrap())
                .expect(&format!("Error find by id in field_type, id: {}", id));
            to_field_type_do(field_type)
        }

        fn query_by_card_type_id(card_type_id: i64) -> Vec<FieldTypeDO> {
            let field_types = dsl::field_types.filter(field_types::card_type_id.eq(card_type_id))
                    .load::<FieldType>(&*CONNECTION.lock().unwrap())
                    .expect(&format!("Error find by card_type_id in field_type, card_type_id: {}", card_type_id));
            to_field_type_do_vec(field_types)
        }
    }

    fn to_field_type_do(field_type: FieldType) -> FieldTypeDO {
        FieldTypeDO {
            id: field_type.id,
            name: field_type.name,
            category: field_type.category,
            card_type_id: field_type.card_type_id
        }
    }

    fn to_field_type_do_vec(field_type_vec: Vec<FieldType>) -> Vec<FieldTypeDO> {
        let mut result = Vec::new();
        for field_type in field_type_vec {
            result.push(to_field_type_do(field_type));
        }
        result
    }

}
