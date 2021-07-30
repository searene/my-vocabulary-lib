mod domain;
mod facade;
mod dependency;
mod infrastructure;

#[macro_use]
extern crate diesel;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
