use leptos::prelude::*;
use leptos_router::components::Redirect;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::app::dashboard::get_user_by_id;
use crate::auth::{Level, check_auth};

#[server]
async fn update_name(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    name: String,
) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

    crate::db::users::update_user_name(&pool, target_id, name)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateUser/{}/{}", target_id, user_id));
    Ok(())
}

#[server]
async fn update_password(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    password: String,
) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

    let hashed = password_auth::generate_hash(password);
    crate::db::users::update_user_password(&pool, target_id, hashed)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateUser/{}/{}", target_id, user_id));
    Ok(())
}

#[server]
async fn update_level(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    level: Level,
) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

    crate::db::users::update_user_level(&pool, target_id, level)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateUser/{}/{}", target_id, user_id));
    Ok(())
}

#[component]
pub fn UpdateUser() -> impl IntoView {
    let auth_check = Resource::new(|| (), |_| check_auth());
    let update_name = ServerAction::<UpdateName>::new();
    let update_password = ServerAction::<UpdatePassword>::new();
    let update_level = ServerAction::<UpdateLevel>::new();

    let params = use_params_map();
    let target_id = move || params.with(|p| p.get("targetId"));
    let target_res = Resource::new(
        move || {
            target_id()
                .as_ref()
                .and_then(|x| Uuid::parse_str(x).ok())
                .unwrap_or(Uuid::nil())
        },
        get_user_by_id,
    );

    let autherized = move || auth_check.get().map(|x| x.is_ok()).unwrap_or(true);
    let user_id = move || {
        auth_check
            .get()
            .transpose()
            .ok()
            .flatten()
            .map(|x| x.to_string())
            .unwrap_or_default()
    };

    let target = move || target_res.get().and_then(|x| x.ok());

    #[component]
    fn Spinner() -> impl IntoView {
        view! {
            <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
                <div class="text-center">
                    <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                    <p class="mt-4 text-gray-600">"جاري التحقق من الهوية..."</p>
                </div>
            </div>
        }
    }

    view! {
        <Suspense fallback=Spinner>
        <Show
            when=autherized
            fallback=move || view! {
                <Redirect path="/login"/>
            }
        >
            <div class="grid grid-cols-1 gap-5 text-center border-5 rounded-lg my-10 mx-5 p-1 md:p-3 lg:p-5">
                <ActionForm action={update_name}>
                    <input class="hidden" type="text" value={user_id} name="user_id"/>
                    <input class="hidden" type="text" value={target_id} name="target_id"/>
                    <div class="grid grid-cols-1 gap-2 my-10">
                        <input
                            class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                            type="text"
                            name="name"
                            id="name"
                            value={move || target().map(|x| x.name)}
                        />
                        <input
                            class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                            type="submit"
                            value="تحديث اسم المستخدم"
                        />
                    </div>
                </ActionForm>
                <ActionForm action={update_level}>
                    <input class="hidden" type="text" value={user_id} name="user_id"/>
                    <input class="hidden" type="text" value={target_id} name="target_id"/>
                    <div class="text-center grid grid-cols-1 gap-2 my-10">
                        <select
                            name="level"
                            id="level"
                            class="p-5 border-2 border-gray-300 rounded-lg text-center block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        >
                          <option value="Admin" selected={move || target().is_some_and(|x| matches!(x.level,Level::Admin))}>"ادمين"</option>
                          <option value="User" selected={move || target().is_some_and(|x| matches!(x.level,Level::User))}>"مستخدم"</option>
                        </select>
                        <input
                            class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                            type="submit"
                            value="تحديث النفوذ"
                        />
                    </div>
                </ActionForm>
                <ActionForm action={update_password}>
                    <input class="hidden" type="text" value={user_id} name="user_id"/>
                    <input class="hidden" type="text" value={target_id} name="target_id"/>
                    <div class="grid grid-cols-1 gap-2 my-10">
                        <input
                            class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                            type="password"
                            name="password"
                            id="password"
                        />
                        <input
                            class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                            type="submit"
                            value="تحديث كلمة السر"
                        />
                    </div>
                </ActionForm>
            </div>
        </Show>
        </Suspense>
    }
}
