use crate::schema::users;
use diesel::PgConnection;
use serde::{Deserialize, Serialize};

use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "users"]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub pass_hash: String,
    pub avatar: Option<String>,
    pub bio: Option<String>,
    pub big_bio: Option<String>,
    pub created_at: chrono::NaiveDateTime,
}

impl User {
    pub fn from <S: Into<String>, T: Into<String>, P: Into<String>> (username: S, email: T, pass: P) -> Self {
        User {
            id: Uuid::new_v4(),
            email: email.into(),
            username: username.into(),
            pass_hash: pass.into(),
            avatar: None,
            bio: None,
            big_bio: None,
            created_at: chrono::Local::now().naive_local(),
        }
    }
}