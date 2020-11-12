use serde::{Serialize, Deserialize};

use diesel::PgConnection;
use diesel::RunQueryDsl;
use diesel::QueryDsl;

use crate::schema::users;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    #[serde(skip)] // IDs are incremental (for now atleast) and should not be shown.
    pub id: i32,
    pub username: String,
    pub email: String,
    #[serde(skip)] // We don't want this in the response now hey.
    pub pass_hash: String,
    pub avatar: String,
    pub bio: String,
    pub big_bio: String,
}



#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub pass_hash: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub big_bio: Option<String>,
}

impl User {
    pub fn create(connection: &PgConnection) -> Result<User, APIError> {
        
    }
}