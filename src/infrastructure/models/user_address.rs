use crate::domain::models::user_address::{CreateUserAddress, UpdateUserAddress, UserAddress};
use crate::infrastructure::schema::user_addresses;
use chrono::{DateTime, Utc};
use diesel::prelude::*;
use uuid::Uuid;

#[derive(Queryable)]
pub struct UserAddressDiesel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

impl From<UserAddress> for UserAddressDiesel {
    fn from(t: UserAddress) -> Self {
        Self {
            id: t.id,
            user_id: t.user_id,
            state: t.state,
            city: t.city,
            pincode: t.pincode,
            house_no: t.house_no,
            area: t.area,
        }
    }
}

#[derive(Insertable)]
#[diesel(table_name = user_addresses)]
pub struct CreateUserAddressDiesel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

#[derive(AsChangeset)]
#[diesel(table_name = user_addresses)]
pub struct UpdateUserAddressDiesel {
    pub id: Uuid,
    pub user_id: Uuid,
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

impl From<UserAddressDiesel> for UserAddress {
    fn from(d: UserAddressDiesel) -> Self {
        Self {
            id: d.id,
            user_id: d.user_id,
            state: d.state,
            city: d.city,
            pincode: d.pincode,
            house_no: d.house_no,
            area: d.area,
        }
    }
}

impl From<CreateUserAddress> for CreateUserAddressDiesel {
    fn from(t: CreateUserAddress) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: t.user_id,
            state: t.state,
            city: t.city,
            pincode: t.pincode,
            house_no: t.house_no,
            area: t.area,
        }
    }
}

impl From<&CreateUserAddress> for CreateUserAddressDiesel {
    fn from(t: &CreateUserAddress) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: t.user_id.clone(),
            state: t.state.clone(),
            city: t.city.clone(),
            pincode: t.pincode.clone(),
            house_no: t.house_no.clone(),
            area: t.area.clone(),
        }
    }
}

impl From<UpdateUserAddress> for UpdateUserAddressDiesel {
    fn from(t: UpdateUserAddress) -> Self {
        Self {
            id: t.id,
            user_id: t.user_id,
            state: t.state,
            city: t.city,
            pincode: t.pincode,
            house_no: t.house_no,
            area: t.area,
        }
    }
}

impl From<&UpdateUserAddress> for UpdateUserAddressDiesel {
    fn from(t: &UpdateUserAddress) -> Self {
        Self {
            id: t.id,
            user_id: t.user_id.clone(),
            state: t.state.clone(),
            city: t.city.clone(),
            pincode: t.pincode.clone(),
            house_no: t.house_no.clone(),
            area: t.area.clone(),
        }
    }
}

impl From<CreateUserAddressDiesel> for UserAddress {
    fn from(d: CreateUserAddressDiesel) -> Self {
        Self {
            id: Uuid::new_v4(),
            user_id: d.user_id,
            state: d.state,
            city: d.city,
            pincode: d.pincode,
            house_no: d.house_no,
            area: d.area,
        }
    }
}
