pub mod card_facade {
    use shaku::Interface;
    use crate::infrastructure::database::field_type_repo::FieldTypeDO;

    enum FieldTypeCategory {
        Text,
        GoogleImage
    }

    impl FieldTypeCategory {
        pub fn from_str(category: &str) -> Result<FieldTypeCategory, String> {
            match category {
                "text" => Ok(FieldTypeCategory::Text),
                "google-image" => Ok(FieldTypeCategory::GoogleImage),
                _ => Err(format!("Unsupported category: {}", category)),
            }
        }
    }

    pub struct FieldTypeVO {

        // field type id
        id: i64,

        // field type category
        category: FieldTypeCategory,

        // field type name
        name: String,
    }

    impl FieldTypeVO {
        pub fn from_field_type_do(field_type_do: &FieldTypeDO) -> FieldTypeVO {
            FieldTypeVO {
                id: field_type_do.id,
                category: FieldTypeCategory::from_str(&field_type_do.category).unwrap(),
                name: field_type_do.name.clone()
            }
        }
    }

    struct SaveCardParamVO {
        word: String,
        book_id: i64,
        card_type_id: i64,
        field_contents: Vec<FieldContentVO>
    }

    struct FieldContentVO {
        field_type_id: i64,
        original_contents: String,
        plain_text_contents: String,
    }

    pub trait CardFacade: Interface {
        fn get_field_types(&self, card_type_id: i64) -> Vec<FieldTypeVO>;
    }

    pub mod implementation {
        use crate::facade::card_facade::{CardFacade, FieldTypeVO};
        use crate::infrastructure::database::field_type_repo::{FieldTypeRepo, FieldTypeDO};
        use shaku::Component;
        use std::sync::Arc;

        #[derive(Component)]
        #[shaku(interface = CardFacade)]
        pub struct CardFacadeImpl {
            #[shaku(inject)]
            field_type_repo: Arc<dyn FieldTypeRepo>
        }

        impl CardFacade for CardFacadeImpl {

            fn get_field_types(self: &Self, card_type_id: i64) -> Vec<FieldTypeVO> {
                let field_type_do_vec: Vec<FieldTypeDO> = self.field_type_repo
                    .query_by_card_type_id(card_type_id);
                field_type_do_vec.iter()
                    .map(&FieldTypeVO::from_field_type_do)
                    .collect()
            }
        }
    }

}