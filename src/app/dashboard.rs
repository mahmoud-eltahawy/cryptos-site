use leptos::prelude::*;
use uuid::Uuid;

use crate::{LoadingSpinner, app::SecureUser, auth::AuthRequired};

pub mod manage_estates;
pub mod manage_user;

#[server]
async fn get_dashboard_stats() -> Result<(usize, usize), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let users_count = crate::db::users::count_users(&app_state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let estates_count = crate::db::estates::count_estates(&app_state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    Ok((users_count as usize, estates_count as usize))
}

#[server]
async fn logout() -> Result<(), ServerFnError> {
    use crate::auth::clear_user_session;
    use tower_sessions::Session;

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
    view! {
        <AuthRequired>
            <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                    <Titles/>
                    <Stats/>
                    <LogoutButton/>
                    <CardsSection/>
                </div>
            </div>
        </AuthRequired>
    }
}

#[component]
fn CardsSection() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-8 max-w-4xl mx-auto">
            <Card
                name="ادارة المستخدمين"
                href="/dashboard/manageUser"
                icon="👥"
                gradient="from-blue-500 to-cyan-500"
            />
            <Card
                name="ادارة العقارات"
                href="/dashboard/manageEstates"
                icon="🏢"
                gradient="from-purple-500 to-pink-500"
            />
        </div>
    }
}

#[component]
fn LogoutButton() -> impl IntoView {
    let logout_action = ServerAction::<Logout>::new();
    view! {
        <div class="flex justify-center mb-8 mt-4">
            <ActionForm action={logout_action}>
                <button
                    type="submit"
                    class="group px-6 py-3 bg-gradient-to-r from-red-500 to-pink-500 text-white font-semibold rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 flex items-center gap-2"
                >
                    <ExitDoorIcon/>
                    "تسجيل الخروج"
                </button>
            </ActionForm>
        </div>

    }
}

#[component]
fn ExitDoorIcon() -> impl IntoView {
    view! {
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17 16l4-4m0 0l-4-4m4 4H7m6 4v1a3 3 0 01-3 3H6a3 3 0 01-3-3V7a3 3 0 013-3h4a3 3 0 013 3v1"></path>
        </svg>
    }
}

#[component]
fn Titles() -> impl IntoView {
    view! {
        <div class="text-center mb-8">
            <h1 class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4">
                "لوحة التحكم"
            </h1>
            <p class="text-gray-600 text-lg">"إدارة العقارات والمستخدمين"</p>
        </div>
    }
}

#[component]
fn Stats() -> impl IntoView {
    let stats_res = Resource::new(|| (), |_| get_dashboard_stats());
    let stats = move || stats_res.get().transpose().ok().flatten();

    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-10 max-w-3xl mx-auto">
            <Suspense fallback=LoadingSpinner>
                <ShowLet
                    some=stats
                    let((user_count,estate_count))
                    fallback=move ||view! {
                        <div class="col-span-1 md:col-span-2 text-center text-sm text-red-600 bg-red-50 border border-red-100 rounded-xl px-4 py-3">
                            "حدث خطأ أثناء تحميل الإحصائيات"
                        </div>
                    }
                >
                    <div class="bg-white/80 backdrop-blur-md rounded-2xl shadow-lg border border-gray-100 p-6 flex items-center justify-between hover:scale-105">
                        <div>
                            <p class="text-sm text-gray-500 mb-1">"إجمالي المستخدمين"</p>
                            <p class="text-3xl font-extrabold text-blue-600">{user_count}</p>
                        </div>
                        <div class="bg-gradient-to-br from-blue-500 to-cyan-500 p-3 rounded-xl text-white">
                            <UserIcon/>
                        </div>
                    </div>
                    <div class="bg-white/80 backdrop-blur-md rounded-2xl shadow-lg border border-gray-100 p-6 flex items-center justify-between hover:scale-105">
                        <div>
                            <p class="text-sm text-gray-500 mb-1">"إجمالي العقارات"</p>
                            <p class="text-3xl font-extrabold text-purple-600">{estate_count}</p>
                        </div>
                        <div class="bg-gradient-to-br from-purple-500 to-pink-500 p-3 rounded-xl text-white">
                            <EstateIcon/>
                        </div>
                    </div>
                </ShowLet>
            </Suspense>
        </div>
    }
}
#[component]
fn UserIcon() -> impl IntoView {
    view! {
        <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
        </svg>
    }
}

#[component]
fn EstateIcon() -> impl IntoView {
    view! {
        <svg class="w-7 h-7" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path
                stroke-linecap="round"
                stroke-linejoin="round"
                stroke-width="2"
                d="M4 5a2 2 0 012-2h3.28a1 1 0 01.948.684l.894 2.683A1 1 0 0013.053 7H18a2 2 0 012 2v9a1 1 0 01-1 1h-5H7H5a1 1 0 01-1-1V5z"></path>
        </svg>
    }
}

#[component]
fn Card(
    href: &'static str,
    name: &'static str,
    icon: &'static str,
    gradient: &'static str,
) -> impl IntoView {
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
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let users = crate::db::users::get_all_users(&app_state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    let xs = users
        .iter()
        .map(|x| (x.id, x.name.clone()))
        .collect::<Vec<_>>();
    Ok(xs)
}

#[server]
async fn get_user_by_id(id: uuid::Uuid) -> Result<SecureUser, ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let user = crate::db::users::get_user_by_id(&app_state.pool, id)
        .await
        .ok()
        .map(SecureUser::from);
    let Some(user) = user else {
        return Err(ServerFnError::ServerError(
            "could not find user with id".to_string(),
        ));
    };
    Ok(user)
}
