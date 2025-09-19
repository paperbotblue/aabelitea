use actix_threadpool::run;
use async_trait::async_trait;
use diesel::prelude::*;
use std::sync::Arc;
use uuid::Uuid;

use crate::domain::models::user_address::{CreateUserAddress, UpdateUserAddress, UserAddress};
use crate::domain::repositories::repository::{QueryParams, RepositoryResult, ResultPaging};
use crate::domain::repositories::user_address::UserAddressQueryParams;
use crate::domain::repositories::user_address::UserAddressRepository;
use crate::infrastructure::databases::postgresql::DBConn;
use crate::infrastructure::error::DieselRepositoryError;
use crate::infrastructure::models::user_address::{
    CreateUserAddressDiesel, UpdateUserAddressDiesel, UserAddressDiesel,
};

pub struct UserAddressDieselRepository {
    pub pool: Arc<DBConn>,
}

impl UserAddressDieselRepository {
    pub fn new(db: Arc<DBConn>) -> Self {
        UserAddressDieselRepository { pool: db }
    }
}

#[async_trait]
impl UserAddressRepository for UserAddressDieselRepository {
    async fn create(&self, new_item: &CreateUserAddress) -> RepositoryResult<UserAddress> {
        use crate::infrastructure::schema::user_addresses::dsl::user_addresses;
        let new_item_diesel: CreateUserAddressDiesel = new_item.into();
        let mut conn = self.pool.get().unwrap();
        let result: UserAddressDiesel = run(move || {
            diesel::insert_into(user_addresses)
                .values(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn update(&self, new_item: &UpdateUserAddress) -> RepositoryResult<UserAddress> {
        use crate::infrastructure::schema::user_addresses::dsl::{id as target_id, user_addresses};
        let new_item_diesel: UpdateUserAddressDiesel = new_item.into();
        let id_val: Uuid = new_item.id;
        let mut conn = self.pool.get().unwrap();
        let result: UserAddressDiesel = run(move || {
            diesel::update(user_addresses.filter(target_id.eq(id_val)))
                .set(new_item_diesel)
                .get_result(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(result.into())
    }

    async fn list(
        &self,
        params: UserAddressQueryParams,
    ) -> RepositoryResult<ResultPaging<UserAddress>> {
        use crate::infrastructure::schema::user_addresses::dsl::user_addresses;
        let pool = self.pool.clone();
        let builder = user_addresses.limit(params.limit()).offset(params.offset());
        let result = run(move || {
            let mut conn = pool.get().unwrap();
            builder.load::<UserAddressDiesel>(&mut conn)
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())?;

        Ok(ResultPaging {
            total: 0,
            items: result.into_iter().map(|v| v.into()).collect(),
        })
    }

    async fn get(&self, item_id: Uuid) -> RepositoryResult<Option<UserAddress>> {
        use crate::infrastructure::schema::user_addresses::dsl::{id, user_addresses};
        let mut conn = self.pool.get().unwrap();
        run(move || {
            user_addresses
                .filter(id.eq(item_id))
                .first::<UserAddressDiesel>(&mut conn)
                .optional()
        })
        .await
        .map_err(|e| DieselRepositoryError::from(e).into_inner())
        .map(|opt| opt.map(|v| v.into())) // map over Option
    }

    async fn delete(&self, item_id: Uuid) -> RepositoryResult<()> {
        use crate::infrastructure::schema::user_addresses::dsl::{id, user_addresses};
        let mut conn = self.pool.get().unwrap();
        run(move || diesel::delete(user_addresses.filter(id.eq(item_id))).execute(&mut conn))
            .await
            .map_err(|e| DieselRepositoryError::from(e).into_inner())?;
        Ok(())
    }
}
