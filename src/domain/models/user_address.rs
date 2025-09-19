use uuid::Uuid;

pub struct UserAddress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

pub struct CreateUserAddress {
    pub user_id: Uuid,
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}

pub struct UpdateUserAddress {
    pub id: Uuid,
    pub user_id: Uuid,
    pub state: Option<String>,
    pub city: Option<String>,
    pub pincode: Option<String>,
    pub house_no: Option<String>,
    pub area: Option<String>,
}
