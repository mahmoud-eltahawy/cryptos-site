use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_router::components::Redirect;
use uuid::Uuid;

use crate::app::SecureUser;

pub mod manage_estates;
pub mod manage_user;

#[server]
async fn check_auth() -> Result<Uuid, ServerFnError> {
    use tower_sessions::Session;
    use crate::auth::require_auth;

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
async fn logout() -> Result<(), ServerFnError> {
    use tower_sessions::Session;
    use crate::auth::clear_user_session;

    let parts = use_context::<axum::http::request::Parts>()
        .ok_or_else(|| ServerFnError::new("No request parts found".to_string()))?;
    let session = parts
        .extensions
        .get::<Session>()
        .ok_or_else(|| ServerFnError::new("No session found".to_string()))?
        .clone();

    clear_user_session(session)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect("/login");
    Ok(())
}

#[component]
pub fn Dashboard() -> impl IntoView {
    let params = use_params_map();
    let user_id = move || params.with(|p| p.get("id"));

    // Check authentication
    let auth_check = Resource::new(|| (), |_| check_auth());
    let logout_action = ServerAction::<Logout>::new();

    view! {
        <Suspense fallback=|| view! {
            <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
                <div class="text-center">
                    <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                    <p class="mt-4 text-gray-600">"جاري التحقق من الهوية..."</p>
                </div>
            </div>
        }>
        {move || {
            auth_check.get().map(|auth_result| {
                match auth_result {
                    Ok(_) => view! {
                        <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
                            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                                <div class="text-center mb-8">
                                    <h1 class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4">
                                        "لوحة التحكم"
                                    </h1>
                                    <p class="text-gray-600 text-lg">"إدارة العقارات والمستخدمين"</p>
                                </div>

                                <div class="flex justify-center mb-8">
                                    <ActionForm action={logout_action}>
                                        <button
                                            type="submit"
                                            class="group px-6 py-3 bg-gradient-to-r from-red-500 to-pink-500 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 flex items-center gap-2"
                                        >
                                            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
                                            </svg>
                                            "تسجيل الخروج"
                                        </button>
                                    </ActionForm>
                                </div>

                                <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
                                    <Card
                                        name="ادارة المستخدمين"
                                        href={format!("/dashboard/manageUser/{}",user_id().unwrap_or("".to_string()))}
                                        icon="👥"
                                        gradient="from-blue-500 to-cyan-500"
                                    />
                                    <Card
                                        name="ادارة العقارات"
                                        href={format!("/dashboard/manageEstates/{}",user_id().unwrap_or("".to_string()))}
                                        icon="🏢"
                                        gradient="from-purple-500 to-pink-500"
                                    />
                                </div>
                            </div>
                        </div>
                    }.into_any(),
                    Err(_) => {
                        view! {
                            <Redirect path="/login"/>
                        }.into_any()
                    }
                }
            })
        }}
        </Suspense>
    }
}

#[component]
fn Card(href: String, name: &'static str, icon: &'static str, gradient: &'static str) -> impl IntoView {
    let gradient_class = format!("bg-gradient-to-br {}", gradient);
    view! {
        <a
            class="group relative overflow-hidden bg-white rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-500 hover:scale-105 border border-gray-100"
            href={href}
        >
            <div class={format!("absolute inset-0 {} opacity-0 group-hover:opacity-10 transition-opacity duration-500", gradient_class)}></div>

            <div class="relative p-8 flex flex-col items-center justify-center h-64">
                <div class={format!("text-6xl mb-6 {} bg-clip-text text-transparent group-hover:scale-110 transition-transform duration-500", gradient_class)}>
                    {icon}
                </div>

                <h2 class="text-2xl md:text-3xl font-bold text-gray-800 mb-4 group-hover:text-blue-600 transition-colors duration-300">
                    {name}
                </h2>

                <div class={format!("w-16 h-1 rounded-full {} group-hover:w-24 transition-all duration-500", gradient_class)}></div>

                <div class="mt-6 flex items-center text-gray-600 group-hover:text-blue-600 transition-colors duration-300">
                    <span class="font-semibold">"انقر للدخول"</span>
                    <svg class="w-5 h-5 mr-2 transform group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 19l-7-7 7-7"></path>
                    </svg>
                </div>
            </div>

            <div class={format!("absolute bottom-0 right-0 w-32 h-32 {} opacity-10 rounded-tl-full transform translate-x-16 translate-y-16 group-hover:translate-x-8 group-hover:translate-y-8 transition-transform duration-500", gradient_class)}></div>
        </a>
    }
}

#[server]
async fn get_users_names() -> Result<Vec<(Uuid, String)>, ServerFnError> {
    let xs = crate::app::DB
        .users
        .lock()
        .unwrap()
        .iter()
        .map(|x| (x.id, x.name.clone()))
        .collect::<Vec<_>>();
    Ok(xs)
}

#[server]
async fn get_user_by_id(id: uuid::Uuid) -> Result<SecureUser, ServerFnError> {
    let user = crate::app::DB
        .users
        .lock()
        .unwrap()
        .iter()
        .find(|x| x.id == id)
        .map(SecureUser::from);
    let Some(user) = user else {
        return Err(ServerFnError::ServerError(
            "could not find user with id".to_string(),
        ));
    };
    Ok(user)
}
