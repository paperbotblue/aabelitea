use std::sync::Arc;
use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::user_address_errors::UserAddressError ;
use crate::domain::models::user_address::{CreateUserAddress, UserAddress, UpdateUserAddress};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user_address::UserAddressQueryParams;
use crate::domain::repositories::user_address::UserAddressRepository;
use crate::domain::services::user_address::UserAddressService;

#[derive(Clone)]
pub struct UserAddressServiceImpl {
    pub repository: Arc<dyn UserAddressRepository>,
}

impl UserAddressServiceImpl {
    pub fn new(repository: Arc<dyn UserAddressRepository>) -> Self {
        Self { repository }
    }
}

#[async_trait]
impl UserAddressService for UserAddressServiceImpl {
    async fn create(&self, item: CreateUserAddress) -> Result<UserAddress, UserAddressError > {
        match self.repository
            .create(&item)
            .await {
              Ok(item) => Ok(item),
              Err(err) => Err(UserAddressError::InternalServerError(err)),
            }
    }

    async fn update(&self, item: UpdateUserAddress) -> Result<UserAddress, UserAddressError > {
        match self.repository
            .update(&item)
            .await {
              Ok(item) => Ok(item),
              Err(err) => Err(UserAddressError::InternalServerError(err)),
            }
    }

    async fn list(&self, params: UserAddressQueryParams) -> Result<ResultPaging<UserAddress>, UserAddressError > {
        match self.repository
            .list(params)
            .await {
              Ok(item) => Ok(item),
              Err(err) => Err(UserAddressError::InternalServerError(err)),
            }
    }

    async fn get(&self, item_id: Uuid) -> Result<UserAddress, UserAddressError > {
        match self.repository
            .get(item_id)
            .await {
              Ok(Some(item)) => Ok(item),
              Ok(None) => Err(UserAddressError::UserAddressDoesNotExist),
              Err(err) => Err(UserAddressError::InternalServerError(err)),
            }
    }

    async fn delete(&self, item_id: Uuid) -> Result<(), UserAddressError > {
        match self.repository
            .delete(item_id)
            .await {
              Ok(item) => Ok(item),
              Err(err) => Err(UserAddressError::InternalServerError(err)),
            }
    }
}
