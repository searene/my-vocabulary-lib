use shaku::module;
use crate::infrastructure::database::implementation::diesel::field_type::DieselFieldTypeRepo;
use lazy_static::lazy_static;
use std::sync::Mutex;
use crate::facade::card_facade::implementation::CardFacadeImpl;

module! {
    pub ShakuModule {
        components = [DieselFieldTypeRepo, CardFacadeImpl],
        providers = []
    }
}

lazy_static! {
    pub static ref MODULE: Mutex<ShakuModule> =
            Mutex::new(get_module());
}

pub fn get_module() -> ShakuModule {
    ShakuModule::builder().build()
}