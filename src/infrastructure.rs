mod database {
    mod repo {
        pub mod field_type_repo {

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

        }
        mod implementation {
            mod diesel {
                mod connection {
                    use std::env;
                    use diesel::{SqliteConnection, Connection};
                    use std::sync::Mutex;
                    use lazy_static::lazy_static;
                    use dotenv::dotenv;

                    lazy_static! {
                        static ref SQLITE_CONNECTION: Mutex<SqliteConnection> =
                                Mutex::new(establish_connection());
                    }

                    fn establish_connection() -> SqliteConnection {
                        dotenv().ok();

                        let database_url = env::var("DATABASE_URL")
                            .expect("DATABASE_URL must be set");
                        SqliteConnection::establish(&database_url)
                            .expect(&format!("Error connecting to {}", database_url))
                    }

                }
                mod table {
                    mod field_type {
                        use crate::infrastructure::database::repo::field_type_repo::{FieldTypeQuery, FieldTypeDO, FieldTypeRepo};
                        use diesel::Queryable;

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
                }
            }
        }
    }
}
pub fn initialize() {

}
