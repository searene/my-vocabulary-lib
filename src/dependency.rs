use shaku::module;
use crate::infrastructure::database::implementation::diesel::field_type::DieselFieldTypeRepo;
use lazy_static::lazy_static;
use std::sync::Mutex;

module! {
    pub ShakuModule {
        components = [DieselFieldTypeRepo],
        providers = []
    }
}

lazy_static! {
    pub static ref module: Mutex<ShakuModule> =
            Mutex::new(get_module());
}

fn get_module() -> ShakuModule {
    ShakuModule::builder().build()
}