mod domain;
mod facade;
mod dependency;
mod infrastructure;

#[macro_use]
extern crate diesel;

#[cfg(test)]
mod tests {

    use std::os::unix::prelude::MetadataExt;

    use shaku::HasComponent;

    use crate::{dependency::get_module, facade::card_facade::CardFacade};

    #[test]
    fn get_field_types_test() {
        let module = get_module();
        let card_facade: &dyn CardFacade = module.resolve_ref();
        let field_types = card_facade.get_field_types(1);
        assert!(field_types.len() >= 0);
    }
}
