use async_trait::async_trait;
use sha2::{Digest, Sha256};
use std::sync::Arc;
use uuid::Uuid;

use crate::api::dto::user::LoginDTO;
use crate::domain::constants;
use crate::domain::errors::user_errors::UserError;
use crate::domain::models::user::{CreateUser, UpdateUser, User};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::repositories::user::UserRepository;
use crate::domain::services::user::UserService;

#[derive(Clone)]
pub struct UserServiceImpl {
    pub repository: Arc<dyn UserRepository>,
}

impl UserServiceImpl {
    pub fn new(repository: Arc<dyn UserRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserService for UserServiceImpl {
    async fn create(&self, mut item: CreateUser) -> Result<User, UserError> {
        item.password_hash = self.hash_password(&item.password_hash);

        match self.repository.create(&item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn login(&self, item: LoginDTO) -> Result<User, UserError> {
        match self.repository.get_by_email(item.email.clone()).await {
            Ok(Some(user)) => {
                if self.verify_password(&item.password, &user.password_hash) {
                    Ok(user)
                } else {
                    Err(UserError::UserNotAuthorised)
                }
            }
            Ok(None) => Err(UserError::UserDoesNotExist),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn update(&self, item: UpdateUser) -> Result<User, UserError> {
        match self.repository.update(&item).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn list(&self, params: UserQueryParams) -> Result<ResultPaging<User>, UserError> {
        match self.repository.list(params).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn get(&self, item_id: Uuid) -> Result<User, UserError> {
        match self.repository.get(item_id).await {
            Ok(Some(item)) => Ok(item),
            Ok(None) => Err(UserError::UserDoesNotExist),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), UserError> {
        match self.repository.delete(item_id).await {
            Ok(item) => Ok(item),
            Err(err) => Err(UserError::InternalServerError(err)),
        }
    }

    fn hash_password(&self, token: &str) -> String {
        let secret_key = "12345678";
        let token_with_key = format!("{}:{}", token, secret_key);
        let mut hasher = Sha256::new();
        hasher.update(token_with_key.as_bytes());
        hex::encode(hasher.finalize())
    }

    fn verify_password(&self, password: &str, password_hash: &str) -> bool {
        self.hash_password(password) == password_hash
    }
}
