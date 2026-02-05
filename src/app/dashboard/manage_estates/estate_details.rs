use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::app::Estate;

#[server]
async fn get_estate_by_id(id: uuid::Uuid) -> Result<Estate, ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

    let estate = crate::db::estates::get_estate_by_id(&pool, id).await.ok();
    let Some(estate) = estate else {
        return Err(ServerFnError::ServerError(
            "could not find estate with id".to_string(),
        ));
    };
    Ok(estate)
}

#[component]
pub fn EstateDetails() -> impl IntoView {
    let params = use_params_map();
    let target_id = move || params.with(|p| p.get("targetId"));

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
        <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 py-12 px-4">
            <Suspense fallback=|| view! {
                <div class="text-center py-12">
                    <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                    <p class="mt-4 text-gray-600">"جاري التحميل..."</p>
                </div>
            }>
                {move || estate().map(|estate| {
                    let Estate {  name, address, image_url,description, price_in_cents, space_in_meters, .. } = estate;
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

                                        <div class="md:col-span-2 bg-gradient-to-br from-green-50 to-emerald-50 p-6 rounded-2xl shadow-md hover:shadow-lg transition-shadow duration-300 border border-green-100">
                                            <p
                                                class="text-center text-lg font-light leading-relaxed text-slate-700 tracking-tight antialiased"
                                            >{description}</p>
                                        </div>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </Suspense>
        </div>
        </Suspense>
    }
}
