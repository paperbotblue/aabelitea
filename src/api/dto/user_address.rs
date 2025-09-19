use chrono::{DateTime, Utc};
use serde::de::Error as DeError;
use serde::{Deserialize, Deserializer, Serialize};
use uuid::Uuid;

use super::validators::user_address::validate_user_address_fields;

#[derive(Serialize)]
pub struct CreateUserAddressDTO {
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct UpdateUserAddressDTO {
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct UserAddressDTO {
    pub id: Uuid,
    pub user_id: Uuid,
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RawUserAddressDTO {
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

impl<'de> Deserialize<'de> for CreateUserAddressDTO {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let raw = RawUserAddressDTO::deserialize(deserializer)?;

        validate_user_address_fields::<D>()?;

        Ok(CreateUserAddressDTO {
            state: raw.state,
            city: raw.city,
            pincode: raw.pincode,
            house_no: raw.house_no,
            area: raw.area,
        })
    }
}
