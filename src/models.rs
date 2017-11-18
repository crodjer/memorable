#[derive(Queryable)]
pub struct Link {
    pub id: i32,
    pub key: String,
    pub url: String,
    pub domain: String,
    pub title: String,
    pub is_custom: bool,
}
