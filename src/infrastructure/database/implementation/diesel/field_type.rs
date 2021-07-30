mod field_type {
    use crate::infrastructure::database::field_type_repo::{FieldTypeQuery, FieldTypeDO, FieldTypeRepo};
    use diesel::Queryable;
    use crate::infrastructure::database::implementation::diesel::connection::SQLITE_CONNECTION;

    #[derive(Queryable)]
    struct FieldType {
        pub id: i64,
        pub name: String,
        pub category: String,
        pub card_type_id: i64,
    }

    struct DieselFieldTypeRepo;

    impl FieldTypeRepo for DieselFieldTypeRepo {
        fn query(field_type_query: FieldTypeQuery) -> Vec<FieldTypeDO> {
            todo!()
        }
    }
}
