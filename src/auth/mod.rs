use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use tower_sessions::Session;
#[cfg(feature = "ssr")]
use uuid::Uuid;

pub const USER_ID_KEY: &str = "user_id";
pub const USER_LEVEL_KEY: &str = "user_level";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "text", rename_all = "PascalCase"))]
pub enum Level {
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
pub async fn get_user_level_from_session(session: Session) -> Option<Level> {
    session.get::<Level>(USER_LEVEL_KEY).await.ok().flatten()
}

#[cfg(feature = "ssr")]
pub async fn set_user_session(
    session: Session,
    user_id: Uuid,
    level: Level,
) -> Result<(), tower_sessions::session::Error> {
    session.insert(USER_ID_KEY, user_id.to_string()).await?;
    session.insert(USER_LEVEL_KEY, level).await?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn clear_user_session(session: Session) -> Result<(), tower_sessions::session::Error> {
    let _ = session.remove::<String>(USER_ID_KEY).await;
    let _ = session.remove::<Level>(USER_LEVEL_KEY).await;
    session.flush().await?;
    Ok(())
}

#[cfg(feature = "ssr")]
pub async fn require_auth(session: Session) -> Result<Uuid, String> {
    get_user_id_from_session(session)
        .await
        .ok_or_else(|| "Unauthorized: Please log in".to_string())
}

#[cfg(feature = "ssr")]
pub async fn require_admin(session: Session) -> Result<Uuid, String> {
    let user_id = get_user_id_from_session(session.clone())
        .await
        .ok_or_else(|| "Unauthorized: Please log in".to_string())?;

    let level = get_user_level_from_session(session)
        .await
        .ok_or_else(|| "Unable to determine user level".to_string())?;

    match level {
        Level::Admin => Ok(user_id),
        Level::User => Err("Forbidden: Admin access required".to_string()),
    }
}
