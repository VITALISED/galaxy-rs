use diesel::PgConnection;

use crate::models::users;
use crate::models::users::User;
use crate::schema::users::dsl::email;

use serde::Deserialize;

use error::{APIError, DBError};

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
        let mut records = users::table
            .filter(email.eq(&self.email))
            .load::<User>(connection)?;

        let user = records
            .pop()
            .ok_or(APIError::DBError(diesel::result::Error::NotFound))?;

        let verify_password = verify(&self.password, &user.password).map_err(|_error| {
            APIError::WrongPassword("Wrong password, check again please".to_string())
        })?;

        if verify_password {
            Ok(user)
        } else {
            Err(APIError::WrongPassword(
                "Wrong password, check again please".to_string(),
            ))
        }
    }
}
