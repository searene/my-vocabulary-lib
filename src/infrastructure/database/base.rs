#[derive(Default)]
pub struct BaseQuery {
    limit: Limit,
}

#[derive(Default)]
pub struct Limit {
    offset: i64,
    limit: Option<i32>
}