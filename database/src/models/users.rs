use crate::schema::*;
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

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable)]
#[table_name = "confirmations"]
pub struct Confirmation {
    pub id: Uuid,
    pub email: String,
    pub expires_at: chrono::NaiveDateTime,
}

impl<T> From<T> for Confirmation where T: Into<String> {
     fn from(email: T) -> Self {
        Confirmation {
            id: Uuid::new_v4(),
            email: email.into(),
            expires_at: chrono::Local::now().naive_local() + chrono::Duration::hours(24),
        }
    }
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