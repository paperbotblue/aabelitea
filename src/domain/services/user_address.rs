use async_trait::async_trait;
use uuid::Uuid;

use crate::domain::errors::user_address_errors::UserAddressError;
use crate::domain::models::user_address::{CreateUserAddress, UserAddress, UpdateUserAddress};
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user_address::UserAddressQueryParams;

#[async_trait]
pub trait UserAddressService: 'static + Sync + Send {
    async fn create(&self, user_address: CreateUserAddress) -> Result<UserAddress, UserAddressError>;
    async fn update(&self, user_address: UpdateUserAddress) -> Result<UserAddress, UserAddressError>;
    async fn list(&self, params: UserAddressQueryParams) -> Result<ResultPaging<UserAddress>, UserAddressError>;
    async fn get(&self, user_address_id: Uuid) -> Result<UserAddress, UserAddressError>;
    async fn delete(&self, user_address_id: Uuid) -> Result<(), UserAddressError>;
}
