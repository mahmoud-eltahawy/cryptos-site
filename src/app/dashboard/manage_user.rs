use leptos::prelude::*;
use leptos_router::components::Redirect;
use uuid::Uuid;

use crate::app::dashboard::get_users_names;

pub mod add_user;
pub mod update_user;

#[server]
async fn check_auth_manage_user() -> Result<Uuid, ServerFnError> {
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

#[server]
async fn remove_user(id: Uuid, target_id: Uuid) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

    crate::db::users::delete_user(&pool, target_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    leptos_axum::redirect(&format!("/dashboard/manageUser/{}", id));
    Ok(())
}

#[component]
pub fn ManageUser() -> impl IntoView {
    let auth_check = Resource::new(|| (), |_| check_auth_manage_user());
    let users_res = Resource::new(|| (), move |_| get_users_names());
    let users = move || users_res.get().and_then(|x| x.ok()).unwrap_or_default();
    let remove_user = ServerAction::<RemoveUser>::new();

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
    #[component]
    fn UsersSpinner() -> impl IntoView {
        view! {
            <div class="text-center py-12">
                <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                <p class="mt-4 text-gray-600">"جاري التحميل..."</p>
            </div>
        }
    }

    view! {
        <Suspense fallback=Spinner>
        <Show
            when=autherized
            fallback=move ||view!{<Redirect path="/login"/>}
        >
            <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 py-12 px-4">
                <div class="max-w-5xl mx-auto">
                    <Titles/>
                    <Suspense fallback=UsersSpinner>
                        <div class="space-y-4 mb-8">
                            <For
                                each={users}
                                key=|x| x.0
                                let((target_id, name))
                            >
                                <ActionForm action={remove_user}>
                                    <div class="bg-white/80 backdrop-blur-sm rounded-xl shadow-lg hover:shadow-2xl transition-all duration-300 p-6 border border-gray-100">
                                        <input class="hidden" name="target_id" value={target_id.to_string()}/>
                                        <input class="hidden" name="id" value={user_id}/>

                                        <div class="flex items-center justify-between gap-4">
                                            <div class="flex items-center gap-4 flex-1">
                                                <div class="bg-gradient-to-br from-blue-500 to-purple-500 rounded-full p-3 shadow-md">
                                                    <UserIcon/>
                                                </div>
                                                <h3 class="text-xl font-bold text-gray-800">{name}</h3>
                                            </div>

                                            <div class="flex items-center gap-3">
                                                <a
                                                    href={move || format!("/dashboard/updateUser/{}",target_id)}
                                                    class="px-5 py-2.5 bg-gradient-to-r from-green-500 to-emerald-500 text-white font-semibold rounded-lg shadow-md hover:shadow-lg hover:scale-105 transition-all duration-300 flex items-center gap-2"
                                                >
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                                    </svg>
                                                    "تحديث"
                                                </a>

                                                <button
                                                    type="submit"
                                                    class="px-5 py-2.5 bg-gradient-to-r from-red-500 to-pink-500 text-white font-semibold rounded-lg shadow-md hover:shadow-lg hover:scale-105 transition-all duration-300 flex items-center gap-2"
                                                >
                                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 7l-.867 12.142A2 2 0 0116.138 21H7.862a2 2 0 01-1.995-1.858L5 7m5 4v6m4-6v6m1-10V4a1 1 0 00-1-1h-4a1 1 0 00-1 1v3M4 7h16"></path>
                                                    </svg>
                                                    "حذف"
                                                </button>
                                            </div>
                                        </div>
                                    </div>
                                </ActionForm>
                            </For>
                        </div>
                    </Suspense>

                    <div class="flex justify-center gap-4">
                        <a
                            href={move || format!("/dashboard/addUser")}
                            class="group px-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold text-lg rounded-xl shadow-lg hover:shadow-2xl hover:scale-105 transition-all duration-300 flex items-center gap-3"
                        >
                            <svg class="w-6 h-6 group-hover:rotate-90 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                            </svg>
                            "إضافة مستخدم جديد"
                        </a>

                        <a
                            href={move || format!("/dashboard/{}", user_id())}
                            class="px-8 py-4 bg-white text-gray-700 font-semibold text-lg rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 border-2 border-gray-200 hover:border-blue-300"
                        >
                            "← العودة إلى لوحة التحكم"
                        </a>
                    </div>
                </div>
            </div>
        </Show>
        </Suspense>
    }
}

#[component]
fn Titles() -> impl IntoView {
    view! {
        <div class="text-center mb-12">
            <h1 class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4 p-4">
                "إدارة المستخدمين"
            </h1>
            <p class="text-gray-600 text-lg">"عرض وتعديل المستخدمين"</p>
        </div>
    }
}

#[component]
fn UserIcon() -> impl IntoView {
    view! {
         <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
             <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
         </svg>
    }
}
