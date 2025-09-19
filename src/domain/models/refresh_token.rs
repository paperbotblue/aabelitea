use actix_web::{FromRequest, HttpMessage};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub struct RefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct CreateRefreshToken {
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

pub struct UpdateRefreshToken {
    pub id: Uuid,
    pub user_id: Uuid,
    pub token: String,
    pub family_id: Uuid,
    pub issued_at: DateTime<Utc>,
    pub expires_at: DateTime<Utc>,
    pub is_revoked: bool,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct JwtClaims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
    pub iat: usize,
}
impl FromRequest for JwtClaims {
    type Error = actix_web::Error;
    type Future = futures_util::future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &actix_web::HttpRequest, _: &mut actix_web::dev::Payload) -> Self::Future {
        // Extract claims from request extensions (assuming your middleware puts them there)
        if let Some(claims) = req.extensions().get::<JwtClaims>() {
            futures_util::future::ready(Ok(claims.clone()))
        } else {
            futures_util::future::ready(Err(actix_web::error::ErrorUnauthorized(
                "Missing or invalid JWT",
            )))
        }
    }
}
