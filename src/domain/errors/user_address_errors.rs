use super::response_code::ApiResponseCode;
use crate::domain::error::{ApiError, RepositoryError} ;
use crate::utils::append_to_file::save_error;
use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum UserAddressError {
    UserAddressAlreadyExists,
    UserAddressDoesNotExist,
    UserAddressNotAuthorised,
    InternalServerError(RepositoryError),
}

// Implement `Display`
impl fmt::Display for UserAddressError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            UserAddressError::UserAddressDoesNotExist => {
                write!(f, "UserAddress Fetching Error: Does Not Exist")
            }

            UserAddressError::UserAddressAlreadyExists => {
                write!(f, "UserAddress Creation Error: Already Exists")
            }

            UserAddressError::UserAddressNotAuthorised => {
                write!(f, "UserAddress Not Authorised")
            }

            UserAddressError::InternalServerError(error) => {
                write!(f, "Internal Server Error(UserAddressError): {}", &error.message)
            }
        }
    }
}

impl From<UserAddressError> for ApiError  {
    fn from(value: UserAddressError) -> Self {
        let code = match value {
            UserAddressError::UserAddressNotAuthorised => ApiResponseCode::Forbidden,
            UserAddressError::UserAddressDoesNotExist => ApiResponseCode::NotFound,
            UserAddressError::UserAddressAlreadyExists => ApiResponseCode::Conflict,
            UserAddressError::InternalServerError(ref e) => {
                save_error(&e.message);
                ApiResponseCode::InternalServerError
            }
        };

        ApiError  {
            message: value.to_string(),
            code: code.status_code(),
        }
    }
}

impl Error for UserAddressError {}
