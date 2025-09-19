use crate::domain::models::user_address::{CreateUserAddress, UpdateUserAddress, UserAddress};
use crate::domain::repositories::repository::{
    QueryParams, RepositoryResult, ResultPaging, DEFAULT_LIMIT, DEFAULT_OFFSET,
};
use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct UserAddressQueryParams {
    pub limit: Option<i64>,
    pub offset: Option<i64>,
    pub title: Option<String>,
}

impl QueryParams for UserAddressQueryParams {
    fn limit(&self) -> i64 {
        self.limit.or(DEFAULT_LIMIT).unwrap_or_default()
    }
    fn offset(&self) -> i64 {
        self.offset.or(DEFAULT_OFFSET).unwrap_or_default()
    }
}

#[async_trait]
pub trait UserAddressRepository: Send + Sync {
    async fn create(&self, new_user_address: &CreateUserAddress) -> RepositoryResult<UserAddress>;
    async fn update(&self, update_user_address: &UpdateUserAddress) -> RepositoryResult<UserAddress>;
    async fn list(&self, params: UserAddressQueryParams) -> RepositoryResult<ResultPaging<UserAddress>>;
    async fn get(&self, user_address_id: Uuid) -> RepositoryResult<Option<UserAddress>>;
    async fn delete(&self, user_address_id: Uuid) -> RepositoryResult<()>;
}

