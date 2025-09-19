use actix_web::{web, HttpResponse, Result};
use std::str::FromStr;
use uuid::Uuid;

use crate::api::dto::user::{CreateUserDTO, LoginDTO, UpdateUserDTO, UserDTO};
use crate::api::dto::user_address::UserAddressDTO;
use crate::domain::error::{ApiError, ApiResponse};
use crate::domain::models::user_address::CreateUserAddress;
use crate::domain::repositories::repository::ResultPaging;
use crate::domain::repositories::user::UserQueryParams;
use crate::domain::services::refresh_token::RefreshTokenService;
use crate::domain::services::user::UserService;
use crate::domain::services::user_address::UserAddressService;

pub async fn create_user_handler(
    user_service: web::Data<dyn UserService>,
    user_address_service: web::Data<dyn UserAddressService>,
    post_data: web::Json<CreateUserDTO>,
) -> Result<ApiResponse<UserDTO>, ApiError> {
    let user = user_service.create(post_data.into_inner().into()).await?;
    let _ = user_address_service
        .create(CreateUserAddress {
            user_id: user.id,
            state: None,
            city: None,
            house_no: None,
            pincode: None,
            area: None,
        })
        .await?;
    Ok(ApiResponse(user.into()))
}

pub async fn login_user_handler(
    user_service: web::Data<dyn UserService>,
    user_address_service: web::Data<dyn UserAddressService>,
    refresh_token_service: web::Data<dyn RefreshTokenService>,
    post_data: web::Json<LoginDTO>,
) -> Result<HttpResponse, ApiError> {
    let user = user_service.login(post_data.into_inner()).await?;
    let user_address = user_address_service.get_by_user_id(user.id).await?;
    let raw_token = refresh_token_service.create_raw_token();
    refresh_token_service
        .create_new_user_refresh_token(user.id, raw_token)
        .await?;
    let jwt_token = refresh_token_service
        .create_user_jwt_token(user.id, user.role.clone().unwrap_or("user".to_string()))
        .await?;

    let cookie = refresh_token_service.build_refresh_token_cookie(raw_token.to_string())?;

    // TODO: dont raw dog this
    let res = HttpResponse::Ok().cookie(cookie).json(serde_json::json!({
        "data": {
            "access_token": jwt_token,
            "user_data": UserDTO::from(user),
            "user_address": UserAddressDTO::from( user_address)
        },
        "success": true,
        "statusCode": 200,
        "msg": "Login successful"
    }));
    Ok(res)
}

pub async fn update_user_handler(
    user_service: web::Data<dyn UserService>,
    post_data: web::Json<UpdateUserDTO>,
) -> Result<ApiResponse<UserDTO>, ApiError> {
    let user = user_service.update(post_data.into_inner().into()).await?;
    Ok(ApiResponse(user.into()))
}

pub async fn list_users_handler(
    user_service: web::Data<dyn UserService>,
    params: web::Query<UserQueryParams>,
) -> Result<ApiResponse<ResultPaging<UserDTO>>, ApiError> {
    let selection = user_service.list(params.into_inner()).await?;
    Ok(ApiResponse(selection.into()))
}

pub async fn get_user_handler(
    user_service: web::Data<dyn UserService>,
    params: web::Path<String>,
) -> Result<ApiResponse<UserDTO>, ApiError> {
    let user = user_service.get(Uuid::from_str(&params)?).await?;
    Ok(ApiResponse(user.into()))
}

pub async fn delete_user_handler(
    user_service: web::Data<dyn UserService>,
    params: web::Path<String>,
) -> Result<HttpResponse, ApiError> {
    user_service.delete(Uuid::from_str(&params)?).await?;
    Ok(HttpResponse::NoContent().finish())
}
