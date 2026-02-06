use crate::app::Estate;
use leptos::prelude::*;

#[server]
async fn get_public_estates() -> Result<Vec<Estate>, ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let res = crate::db::estates::get_all_estates(&app_state.pool)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    Ok(res)
}

#[component]
pub fn PublicEstates() -> impl IntoView {
    let estates_res = Resource::new(|| (), move |_| get_public_estates());
    let estates = move || estates_res.get().and_then(|x| x.ok()).unwrap_or_default();

    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 py-12 px-4">
            <div class="max-w-7xl mx-auto">
                <div class="text-center mb-12">
                    <h1 class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4">
                        "العقارات"
                    </h1>
                    <p class="text-gray-600 text-lg">"تصفح العقارات المتاحة"</p>
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
                                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2"></path>
                                                </svg>
                                                <span class="font-bold">{format!("{:.2}", price_in_cents as f32 / 100.0)}" ج"</span>
                                            </div>
                                        </div>
                                    </div>

                                    <div class="flex flex-wrap gap-3">
                                        <a
                                            href={format!("/dashboard/estateDetails/{}", id)}
                                            class="flex-1 px-4 py-2.5 bg-gradient-to-r from-blue-500 to-cyan-500 text-white font-semibold rounded-lg shadow-md hover:shadow-lg hover:scale-105 transition-all duration-300 text-center"
                                        >
                                            "التفاصيل"
                                        </a>
                                    </div>
                                </div>
                            </div>
                        </For>
                    </div>
                </Suspense>
            </div>
        </div>
    }
}
