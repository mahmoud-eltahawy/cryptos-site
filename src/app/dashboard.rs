use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::app::SecureUser;

pub mod manage_estates;
pub mod manage_user;

#[component]
pub fn Dashboard() -> impl IntoView {
    let params = use_params_map();
    let user_id = move || params.with(|p| p.get("id"));
    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 py-12">
                <div class="text-center mb-12">
                    <h1 class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4">
                        "لوحة التحكم"
                    </h1>
                    <p class="text-gray-600 text-lg">"إدارة العقارات والمستخدمين"</p>
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
