use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use sqlx::FromRow;
use uuid::Uuid;

#[cfg(feature = "ssr")]
use chrono::{DateTime, Utc};

use crate::auth::Level;

#[derive(Debug, Clone)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub password: String,
    pub level: Level,
    #[cfg(feature = "ssr")]
    pub created_at: DateTime<Utc>,
    #[cfg(feature = "ssr")]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[cfg_attr(feature = "ssr", derive(FromRow))]
pub struct Estate {
    pub id: Uuid,
    pub name: String,
    pub address: String,
    pub image_url: String,
    pub price_in_cents: i64,
    pub space_in_meters: i32,
    #[cfg(feature = "ssr")]
    #[serde(skip)]
    pub created_at: DateTime<Utc>,
    #[cfg(feature = "ssr")]
    #[serde(skip)]
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecureUser {
    pub id: Uuid,
    pub name: String,
    pub level: Level,
}

impl From<&User> for SecureUser {
    fn from(user: &User) -> Self {
        Self {
            id: user.id,
            name: user.name.clone(),
            level: user.level.clone(),
        }
    }
}

impl From<User> for SecureUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            level: user.level,
        }
    }
}
