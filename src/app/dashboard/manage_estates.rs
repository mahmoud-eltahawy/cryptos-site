use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use leptos_router::components::Redirect;

use crate::app::Estate;

pub mod add_estate;
pub mod update_estate;
pub mod estate_details {
    use leptos::prelude::*;
    use leptos_router::hooks::use_params_map;
    use leptos_router::components::Redirect;
    use uuid::Uuid;

    use crate::app::Estate;

    #[server]
    async fn check_auth_estate_details() -> Result<Uuid, ServerFnError> {
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
    async fn get_estate_by_id(id: uuid::Uuid) -> Result<Estate, ServerFnError> {
        let pool = use_context::<sqlx::PgPool>()
            .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

        let estate = crate::db::estates::get_estate_by_id(&pool, id)
            .await
            .ok();
        let Some(estate) = estate else {
            return Err(ServerFnError::ServerError(
                "could not find estate with id".to_string(),
            ));
        };
        Ok(estate)
    }

    #[component]
    pub fn EstateDetails() -> impl IntoView {
        let auth_check = Resource::new(|| (), |_| check_auth_estate_details());
        let params = use_params_map();
        let target_id = move || params.with(|p| p.get("targetId"));
        let user_id = move || params.with(|p| p.get("userId")).unwrap_or(String::new());

        let estate_res = Resource::new(
            move || {
                target_id()
                    .as_ref()
                    .and_then(|x| Uuid::parse_str(x).ok())
                    .unwrap_or(Uuid::nil())
            },
            get_estate_by_id,
        );

        let estate = move || estate_res.get().and_then(|x| x.ok());

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
            <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 py-12 px-4">
                <Suspense fallback=|| view! {
                    <div class="text-center py-12">
                        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                        <p class="mt-4 text-gray-600">"جاري التحميل..."</p>
                    </div>
                }>
                    {move || estate().map(|estate| {
                        let Estate { id, name, address, image_url, price_in_cents, space_in_meters, .. } = estate;
                        view! {
                            <div class="max-w-5xl mx-auto">
                                <div class="bg-white/90 backdrop-blur-lg rounded-3xl shadow-2xl overflow-hidden border border-gray-100">
                                    <div class="relative h-96 overflow-hidden">
                                        <img
                                            class="w-full h-full object-cover transform hover:scale-110 transition-transform duration-700"
                                            src={image_url.clone()}
                                            alt={name.clone()}
                                        />
                                        <div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent"></div>
                                        <div class="absolute bottom-0 left-0 right-0 p-8">
                                            <h1 class="text-4xl md:text-5xl font-bold text-white mb-2">{name.clone()}</h1>
                                            <div class="flex items-center text-white/90 gap-2">
                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                                </svg>
                                                <span class="text-lg">{address.clone()}</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="p-8">
                                        <div class="grid grid-cols-1 md:grid-cols-2 gap-6 mb-8">
                                            <div class="bg-gradient-to-br from-blue-50 to-cyan-50 p-6 rounded-2xl shadow-md hover:shadow-lg transition-shadow duration-300 border border-blue-100">
                                                <div class="flex items-center gap-3 mb-3">
                                                    <div class="bg-gradient-to-br from-blue-500 to-cyan-500 p-3 rounded-xl shadow-md">
                                                        <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h4a1 1 0 011 1v7a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM14 5a1 1 0 011-1h4a1 1 0 011 1v7a1 1 0 01-1 1h-4a1 1 0 01-1-1V5zM4 16a1 1 0 011-1h4a1 1 0 011 1v3a1 1 0 01-1 1H5a1 1 0 01-1-1v-3zM14 16a1 1 0 011-1h4a1 1 0 011 1v3a1 1 0 01-1 1h-4a1 1 0 01-1-1v-3z"></path>
                                                        </svg>
                                                    </div>
                                                    <h3 class="text-xl font-bold text-gray-800">"المساحة"</h3>
                                                </div>
                                                <p class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-cyan-600 bg-clip-text text-transparent">
                                                    {space_in_meters}" متر²"
                                                </p>
                                            </div>

                                            <div class="bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-2xl shadow-md hover:shadow-lg transition-shadow duration-300 border border-green-100">
                                                <div class="flex items-center gap-3 mb-3">
                                                    <div class="bg-gradient-to-br from-green-500 to-emerald-500 p-3 rounded-xl shadow-md">
                                                        <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                        </svg>
                                                    </div>
                                                    <h3 class="text-xl font-bold text-gray-800">"السعر"</h3>
                                                </div>
                                                <p class="text-3xl font-bold bg-gradient-to-r from-green-600 to-emerald-600 bg-clip-text text-transparent">
                                                    {format!("{:.2}", price_in_cents as f32 / 100.0)}" جنيه"
                                                </p>
                                            </div>
                                        </div>

                                        <div class="bg-gradient-to-br from-purple-50 to-pink-50 p-6 rounded-2xl shadow-md border border-purple-100 mb-8">
                                            <div class="flex items-center gap-3 mb-3">
                                                <div class="bg-gradient-to-br from-purple-500 to-pink-500 p-3 rounded-xl shadow-md">
                                                    <svg class="w-6 h-6 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                                                    </svg>
                                                </div>
                                                <h3 class="text-xl font-bold text-gray-800">"معرف العقار"</h3>
                                            </div>
                                            <p class="text-sm text-gray-600 font-mono bg-white/50 px-4 py-2 rounded-lg">{id.to_string()}</p>
                                        </div>

                                        <div class="flex flex-wrap gap-4 justify-center">
                                            <a
                                                href={format!("/dashboard/updateEstate/{}/{}", id, user_id())}
                                                class="group px-8 py-3 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold rounded-xl shadow-lg hover:shadow-2xl hover:scale-105 transition-all duration-300 flex items-center gap-3"
                                            >
                                                <svg class="w-5 h-5 group-hover:rotate-12 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M11 5H6a2 2 0 00-2 2v11a2 2 0 002 2h11a2 2 0 002-2v-5m-1.414-9.414a2 2 0 112.828 2.828L11.828 15H9v-2.828l8.586-8.586z"></path>
                                                </svg>
                                                "تعديل العقار"
                                            </a>
                                            <a
                                                href={format!("/dashboard/manageEstates/{}", user_id())}
                                                class="px-8 py-3 bg-white text-gray-700 font-semibold rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 border-2 border-gray-200 hover:border-blue-300 flex items-center gap-2"
                                            >
                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M10 19l-7-7m0 0l7-7m-7 7h18"></path>
                                                </svg>
                                                "العودة"
                                            </a>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        }
                    })}
                </Suspense>
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
}

#[server]
async fn check_auth_manage_estates() -> Result<uuid::Uuid, ServerFnError> {
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
async fn remove_estate(id: uuid::Uuid, target_id: uuid::Uuid) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

    crate::db::estates::delete_estate(&pool, target_id)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    leptos_axum::redirect(&format!("/dashboard/manageEstates/{}", id));
    Ok(())
}

#[server]
async fn get_estates() -> Result<Vec<Estate>, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

    let res = crate::db::estates::get_all_estates(&pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(res)
}

#[component]
pub fn ManageEstates() -> impl IntoView {
    let auth_check = Resource::new(|| (), |_| check_auth_manage_estates());
    let estates_res = Resource::new(|| (), move |_| get_estates());
    let estates = move || estates_res.get().and_then(|x| x.ok()).unwrap_or_default();
    let remove_estate = ServerAction::<RemoveEstate>::new();

    let params = use_params_map();
    let user_id = move || params.with(|p| p.get("id"));

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
        <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 py-12 px-4">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-12">
                    <h1 class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4">
                        "إدارة العقارات"
                    </h1>
                    <p class="text-gray-600 text-lg">"عرض وتعديل العقارات المتاحة"</p>
                </div>

                <Suspense fallback=|| view! {
                    <div class="text-center py-12">
                        <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                        <p class="mt-4 text-gray-600">"جاري التحميل..."</p>
                    </div>
                }>
                    <div class="grid grid-cols-1 lg:grid-cols-2 gap-8 mb-8">
                        <For
                            each={estates}
                            key=|x| x.id
                            let(Estate { id, name, address, image_url, price_in_cents, space_in_meters, .. })
                        >
                            <div class="group bg-white/90 backdrop-blur-sm rounded-2xl shadow-lg hover:shadow-2xl transition-all duration-500 overflow-hidden border border-gray-100 hover:scale-[1.02]">
                                <div class="relative h-64 overflow-hidden">
                                    <img
                                        class="w-full h-full object-cover transform group-hover:scale-110 transition-transform duration-700"
                                        src={image_url}
                                        alt={name.clone()}
                                    />
                                    <div class="absolute inset-0 bg-gradient-to-t from-black/60 to-transparent opacity-0 group-hover:opacity-100 transition-opacity duration-300"></div>
                                </div>

                                <div class="p-6">
                                    <h2 class="text-2xl font-bold text-gray-800 mb-3 group-hover:text-blue-600 transition-colors duration-300">
                                        {name}
                                    </h2>

                                    <div class="space-y-3 mb-6">
                                        <div class="flex items-center text-gray-600 gap-2">
                                            <svg class="w-5 h-5 text-blue-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                            </svg>
                                            <span>{address}</span>
                                        </div>

                                        <div class="flex items-center justify-between">
                                            <div class="flex items-center text-gray-700 gap-2">
                                                <svg class="w-5 h-5 text-purple-500" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h4a1 1 0 011 1v7a1 1 0 01-1 1H5a1 1 0 01-1-1V5z"></path>
                                                </svg>
                                                <span class="font-semibold">{space_in_meters}" متر²"</span>
                                            </div>

                                            <div class="flex items-center text-green-600 gap-2">
                                                <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                                </svg>
                                                <span class="font-bold">{format!("{:.2}", price_in_cents as f32 / 100.0)}" ج"</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="flex flex-wrap gap-3">
                                        <a
                                            href={move || format!("/dashboard/estateDetails/{}/{}",id,user_id().unwrap_or("".to_string()))}
                                            class="flex-1 px-4 py-2.5 bg-gradient-to-r from-blue-500 to-cyan-500 text-white font-semibold rounded-lg shadow-md hover:shadow-lg hover:scale-105 transition-all duration-300 text-center"
                                        >
                                            "التفاصيل"
                                        </a>

                                        <a
                                            href={move || format!("/dashboard/updateEstate/{}/{}",id,user_id().unwrap_or("".to_string()))}
                                            class="flex-1 px-4 py-2.5 bg-gradient-to-r from-green-500 to-emerald-500 text-white font-semibold rounded-lg shadow-md hover:shadow-lg hover:scale-105 transition-all duration-300 text-center"
                                        >
                                            "تحديث"
                                        </a>

                                        <div class="flex-1">
                                            <ActionForm action={remove_estate}>
                                                <input class="hidden" name="id" value={user_id}/>
                                                <input class="hidden" name="target_id" value={id.to_string()}/>
                                                <button
                                                    type="submit"
                                                    class="w-full px-4 py-2.5 bg-gradient-to-r from-red-500 to-pink-500 text-white font-semibold rounded-lg shadow-md hover:shadow-lg hover:scale-105 transition-all duration-300"
                                                >
                                                    "حذف"
                                                </button>
                                            </ActionForm>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </For>
                    </div>
                </Suspense>

                <div class="flex justify-center gap-4 mt-12">
                    <a
                        href={move || format!("/dashboard/addEstate/{}",user_id().unwrap_or("".to_string()))}
                        class="group px-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold text-lg rounded-xl shadow-lg hover:shadow-2xl hover:scale-105 transition-all duration-300 flex items-center gap-3"
                    >
                        <svg class="w-6 h-6 group-hover:rotate-90 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 4v16m8-8H4"></path>
                        </svg>
                        "إضافة عقار جديد"
                    </a>

                    <a
                        href={move || format!("/dashboard/{}", user_id().unwrap_or("".to_string()))}
                        class="px-8 py-4 bg-white text-gray-700 font-semibold text-lg rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 border-2 border-gray-200 hover:border-blue-300"
                    >
                        "← العودة إلى لوحة التحكم"
                    </a>
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
