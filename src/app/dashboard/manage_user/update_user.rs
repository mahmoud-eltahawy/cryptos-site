use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::app::SecureUser;
use crate::app::dashboard::get_user_by_id;
use crate::auth::{AuthRequired, Level};

#[server]
async fn update_name(target_id: uuid::Uuid, name: String) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::users::update_user_name(&app_state.pool, target_id, name)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateUser/{}", target_id));
    Ok(())
}

#[server]
async fn update_password(target_id: uuid::Uuid, password: String) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let hashed = password_auth::generate_hash(password);
    crate::db::users::update_user_password(&app_state.pool, target_id, hashed)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateUser/{}", target_id));
    Ok(())
}

#[server]
async fn update_level(target_id: uuid::Uuid, level: Level) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::users::update_user_level(&app_state.pool, target_id, level)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateUser/{}", target_id));
    Ok(())
}

#[component]
pub fn UpdateUser() -> impl IntoView {
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

    let target = move || target_res.get().and_then(|x| x.ok());

    view! {
        <AuthRequired>
            <Suspense>
                <ShowLet
                    some=target
                    let(SecureUser{id,name,level})
                >
                    <div class="grid grid-cols-1 gap-5 text-center border-5 rounded-lg my-10 mx-5 p-1 md:p-3 lg:p-5">
                        <UpdateName id name/>
                        <UpdateLevel id level/>
                        <UpdatePassword id/>
                    </div>
                </ShowLet>
            </Suspense>
        </AuthRequired>
        <GoBack/>
    }
}

#[component]
fn UpdatePassword(id: uuid::Uuid) -> impl IntoView {
    let update_password = ServerAction::<UpdatePassword>::new();
    view! {
        <ActionForm action={update_password}>
            <input class="hidden" type="text" value={id.to_string()} name="target_id"/>
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
    }
}

#[component]
fn UpdateLevel(id: uuid::Uuid, level: Level) -> impl IntoView {
    let update_level = ServerAction::<UpdateLevel>::new();
    view! {
        <ActionForm action={update_level}>
            <input class="hidden" type="text" value={id.to_string()} name="target_id"/>
            <div class="text-center grid grid-cols-1 gap-2 my-10">
                <select
                    name="level"
                    id="level"
                    class="p-5 border-2 border-gray-300 rounded-lg text-center block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                >
                  <option value="Admin" selected={matches!(level,Level::Admin)}>"ادمين"</option>
                  <option value="User" selected={matches!(level,Level::User)}>"مستخدم"</option>
                </select>
                <input
                    class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    type="submit"
                    value="تحديث النفوذ"
                />
            </div>
        </ActionForm>
    }
}

#[component]
fn UpdateName(id: uuid::Uuid, name: String) -> impl IntoView {
    let update_name = ServerAction::<UpdateName>::new();
    view! {
        <ActionForm action={update_name}>
            <input class="hidden" type="text" value={id.to_string()} name="target_id"/>
            <div class="grid grid-cols-1 gap-2 my-10">
                <input
                    class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                    type="text"
                    name="name"
                    id="name"
                    value={name}
                />
                <input
                    class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    type="submit"
                    value="تحديث اسم المستخدم"
                />
            </div>
        </ActionForm>
    }
}

#[component]
fn GoBack() -> impl IntoView {
    view! {
        <div class="mt-5 grid-cols-1 text-center m-10">
            <a
                href="/dashboard/manageUser"
                class="px-8 py-4 bg-white text-gray-700 font-semibold text-lg rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 border-2 border-gray-200 hover:border-blue-300"
            >"العودة إلى إدارة المستخدمين"</a>
        </div>
    }
}
