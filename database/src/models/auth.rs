use diesel::PgConnection;

use crate::models::users::User;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct RegisterUser {
    pub email: String,
    pub password: String,
    pub password_confirmation: String,
}

impl RegisterUser {
    pub fn validate(self) -> Result<RegisterUser, APIError> {
        if self.password == self.password_confirmation {
            Ok(self)
        } /*else {
            Err(
                error code comes later
            )
        }*/
    }
}



#[derive(Deserialize)]
pub struct AuthUser {
    email: String,
    password: String,
}

impl AuthUser {
    pub fn login(&self, connection: &PgConnection) -> Result<User, APIError> {
        
    }
}