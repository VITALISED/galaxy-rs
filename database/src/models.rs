#[derive(Queryable)]
pub struct User {
    pub id: u64,
    pub username: String,
    pub pass_hash: String,
    pub avatar: String,
    pub bio: String,
    pub big_bio: String,
}
