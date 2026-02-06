use leptos::prelude::*;

use leptos_router::components::Redirect;
use serde::{Deserialize, Serialize};
#[cfg(feature = "ssr")]
use tower_sessions::Session;
#[cfg(feature = "ssr")]
use uuid::Uuid;

use std::fmt::Display;

pub const USER_ID_KEY: &str = "user_id";
pub const USER_LEVEL_KEY: &str = "user_level";

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[cfg_attr(feature = "ssr", derive(sqlx::Type))]
#[cfg_attr(feature = "ssr", sqlx(type_name = "text", rename_all = "PascalCase"))]
pub enum Level {
    Admin,
    User,
}

impl Display for Level {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let res = match self {
            Level::Admin => "Admin",
            Level::User => "User",
        };
        write!(f, "{res}")
    }
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

#[server]
pub async fn check_auth() -> Result<uuid::Uuid, ServerFnError> {
    use crate::auth::require_auth;
    use tower_sessions::Session;

    let parts = use_context::<axum::http::request::Parts>()
        .ok_or_else(|| ServerFnError::new("No request parts found".to_string()))?;
    let session = parts
        .extensions
        .get::<Session>()
        .ok_or_else(|| ServerFnError::new("No session found".to_string()))?
        .clone();

    require_auth(session)
        .await
        .map_err(|e| ServerFnError::ServerError(e))
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

#[component]
pub fn AuthRequired<C>(children: TypedChildrenFn<C>, redirect: String) -> impl IntoView
where
    C: IntoView + 'static,
{
    let auth_check = Resource::new(|| (), |_| check_auth());
    let autherized = move || auth_check.get().map(|x| x.is_ok()).unwrap_or(true);
    let children = children.into_inner();

    let fallback = move || {
        view! {
            <Redirect path={redirect.clone()}/>
        }
    };
    view! {
        <Suspense fallback=AuthSpinner>
            <Show
                when={autherized}
                fallback=fallback
            >
                {children()}
            </Show>
        </Suspense>

    }
}

#[component]
fn AuthSpinner() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
            <div class="text-center">
                <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                <p class="mt-4 text-gray-600">"جاري التحقق من الهوية..."</p>
            </div>
        </div>
    }
}
