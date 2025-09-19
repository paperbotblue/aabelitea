use uuid::Uuid;

use crate::{
    api::dto::user_address::{CreateUserAddressDTO, UpdateUserAddressDTO, UserAddressDTO},
    domain::{
        models::user_address::{CreateUserAddress, UpdateUserAddress, UserAddress},
        repositories::repository::ResultPaging,
    },
};

impl From<UserAddress> for UserAddressDTO {
    fn from(value: UserAddress) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            state: value.state,
            city: value.city,
            pincode: value.pincode,
            house_no: value.house_no,
            area: value.area,
        }
    }
}

impl From<CreateUserAddressDTO> for CreateUserAddress {
    fn from(value: CreateUserAddressDTO) -> Self {
        Self {
            // TODO: this is gonna cause issues down the line
            user_id: Uuid::new_v4(),
            state: value.state,
            city: value.city,
            pincode: value.pincode,
            house_no: value.house_no,
            area: value.area,
        }
    }
}

impl From<CreateUserAddress> for CreateUserAddressDTO {
    fn from(value: CreateUserAddress) -> Self {
        Self {
            state: value.state,
            city: value.city,
            pincode: value.pincode,
            house_no: value.house_no,
            area: value.area,
        }
    }
}

impl From<UpdateUserAddress> for UpdateUserAddressDTO {
    fn from(value: UpdateUserAddress) -> Self {
        Self {
            state: value.state.unwrap_or("null".to_string()),
            city: value.city,
            pincode: value.pincode,
            house_no: value.house_no,
            area: value.area,
        }
    }
}

impl From<ResultPaging<UserAddress>> for ResultPaging<UserAddressDTO> {
    fn from(value: ResultPaging<UserAddress>) -> Self {
        Self {
            total: value.total,
            items: value.items.into_iter().map(Into::into).collect(),
        }
    }
}
