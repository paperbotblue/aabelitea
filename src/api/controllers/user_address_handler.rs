use std::str::FromStr;

use actix_web::{web, HttpResponse, Result};
use uuid::Uuid;

use crate::api::dto::user_address::{CreateUserAddressDTO, UpdateUserAddressDTO, UserAddressDTO};
use crate::domain::error::ApiError;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user_address::UserAddressQueryParams;
use crate::domain::services::user_address::UserAddressService;

pub async fn create_user_address_handler(
    user_address_service: web::Data<dyn UserAddressService>,
    post_data: web::Json<CreateUserAddressDTO>,
) -> Result<web::Json<UserAddressDTO>, ApiError> {
    let user_address = user_address_service
        .create(post_data.into_inner().into())
        .await?;
    Ok(web::Json(user_address.into()))
}

pub async fn update_user_address_handler(
    user_address_service: web::Data<dyn UserAddressService>,
    post_data: web::Json<UpdateUserAddressDTO>,
) -> Result<web::Json<UserAddressDTO>, ApiError> {
    let user_address = user_address_service
        .update(post_data.into_inner().into())
        .await?;
    Ok(web::Json(user_address.into()))
}

pub async fn list_user_addresss_handler(
    user_address_service: web::Data<dyn UserAddressService>,
    params: web::Query<UserAddressQueryParams>,
) -> Result<web::Json<ResultPaging<UserAddressDTO>>, ApiError> {
    let selection = user_address_service.list(params.into_inner()).await?;
    Ok(web::Json(selection.into()))
}

pub async fn get_user_address_handler(
    user_address_service: web::Data<dyn UserAddressService>,
    params: web::Path<String>,
) -> Result<web::Json<UserAddressDTO>, ApiError> {
    let user_address = user_address_service.get(Uuid::from_str(&params)?).await?;
    Ok(web::Json(user_address.into()))
}

pub async fn delete_user_address_handler(
    user_address_service: web::Data<dyn UserAddressService>,
    params: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    user_address_service
        .delete(Uuid::from_str(&params)?)
        .await?;
    Ok(HttpResponse::NoContent().finish())
}
