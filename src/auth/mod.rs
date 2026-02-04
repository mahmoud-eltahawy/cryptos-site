use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use tower_sessions::Session;
use uuid::Uuid;

pub const USER_ID_KEY: &str = "user_id";
pub const USER_LEVEL_KEY: &str = "user_level";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum UserLevel {
    Admin,
    User,
}

#[cfg(feature = "ssr")]
pub async fn get_user_id_from_session(session: Session) -> Option<Uuid> {
    session
        .get::<String>(USER_ID_KEY)
        .await
        .ok()
        .flatten()
        .and_then(|id| Uuid::parse_str(&id).ok())
}

#[cfg(feature = "ssr")]
pub async fn get_user_level_from_session(session: Session) -> Option<UserLevel> {
    session
        .get::<UserLevel>(USER_LEVEL_KEY)
        .await
        .ok()
        .flatten()
}

#[cfg(feature = "ssr")]
pub async fn set_user_session(
    session: Session,
    user_id: Uuid,
    level: UserLevel,
) -> Result<(), tower_sessions::session::Error> {
    session.insert(USER_ID_KEY, user_id.to_string()).await?;
    session.insert(USER_LEVEL_KEY, level).await?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn clear_user_session(session: Session) -> Result<(), tower_sessions::session::Error> {
    let _ = session.remove::<String>(USER_ID_KEY).await;
    let _ = session.remove::<UserLevel>(USER_LEVEL_KEY).await;
    session.flush().await?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn require_auth(session: Session) -> Result<Uuid, String> {
    get_user_id_from_session(session)
        .await
        .ok_or_else(|| "Unauthorized: Please log in".to_string())
}
