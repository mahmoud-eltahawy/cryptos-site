use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

#[server]
async fn add_estate(
    id: Uuid,
    name: String,
    address: String,
    image_url: String,
    price_in_cents: usize,
    space_in_meters: usize,
) -> Result<(), ServerFnError> {
    use crate::app::DB;

    DB.estates.lock().unwrap().push(crate::app::Estate {
        id: Uuid::new_v4(),
        name,
        address,
        image_url,
        price_in_cents,
        space_in_meters,
    });
    leptos_axum::redirect(&format!("/dashboard/manageEstates/{}", id));
    Ok(())
}

#[component]
pub fn AddEstate() -> impl IntoView {
    let params = use_params_map();
    let user_id = move || params.with(|p| p.get("id"));

    let add_estate = ServerAction::<AddEstate>::new();

    view! {
        <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 py-12 px-4">
            <div class="max-w-3xl mx-auto">
                <div class="bg-white/90 backdrop-blur-lg rounded-3xl shadow-2xl overflow-hidden border border-gray-100">
                    <div class="bg-gradient-to-r from-blue-600 to-purple-600 p-8 text-white">
                        <div class="flex items-center gap-4">
                            <div class="bg-white/20 backdrop-blur-sm p-4 rounded-2xl">
                                <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M19 21V5a2 2 0 00-2-2H7a2 2 0 00-2 2v16m14 0h2m-2 0h-5m-9 0H3m2 0h5M9 7h1m-1 4h1m4-4h1m-1 4h1m-5 10v-5a1 1 0 011-1h2a1 1 0 011 1v5m-4 0h4"></path>
                                </svg>
                            </div>
                            <div>
                                <h1 class="text-3xl font-bold">"إضافة عقار جديد"</h1>
                                <p class="text-blue-100 mt-1">"أدخل تفاصيل العقار الجديد"</p>
                            </div>
                        </div>
                    </div>

                    <div class="p-8">
                        <ActionForm action={add_estate}>
                            <input class="hidden" type="text" value={user_id} name="id"/>

                            <div class="space-y-6">
                                <div class="group">
                                    <label
                                        class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                                        for="name"
                                    >
                                        <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M7 7h.01M7 3h5c.512 0 1.024.195 1.414.586l7 7a2 2 0 010 2.828l-7 7a2 2 0 01-2.828 0l-7-7A1.994 1.994 0 013 12V7a4 4 0 014-4z"></path>
                                        </svg>
                                        "اسم العقار"
                                    </label>
                                    <input
                                        class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                                        type="text"
                                        name="name"
                                        id="name"
                                        placeholder="مثال: فيلا فاخرة في التجمع الخامس"
                                        required
                                    />
                                </div>

                                <div class="group">
                                    <label
                                        class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                                        for="address"
                                    >
                                        <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                        </svg>
                                        "العنوان"
                                    </label>
                                    <input
                                        class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                                        type="text"
                                        name="address"
                                        id="address"
                                        placeholder="مثال: التجمع الخامس، القاهرة الجديدة"
                                        required
                                    />
                                </div>

                                <div class="group">
                                    <label
                                        class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                                        for="image_url"
                                    >
                                        <svg class="w-5 h-5 text-pink-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                                        </svg>
                                        "رابط الصورة"
                                    </label>
                                    <input
                                        class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                                        type="url"
                                        name="image_url"
                                        id="image_url"
                                        placeholder="https://example.com/image.jpg"
                                        required
                                    />
                                </div>

                                <div class="grid grid-cols-1 md:grid-cols-2 gap-6">
                                    <div class="group">
                                        <label
                                            class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                                            for="space_in_meters"
                                        >
                                            <svg class="w-5 h-5 text-cyan-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 5a1 1 0 011-1h4a1 1 0 011 1v7a1 1 0 01-1 1H5a1 1 0 01-1-1V5zM14 5a1 1 0 011-1h4a1 1 0 011 1v7a1 1 0 01-1 1h-4a1 1 0 01-1-1V5zM4 16a1 1 0 011-1h4a1 1 0 011 1v3a1 1 0 01-1 1H5a1 1 0 01-1-1v-3zM14 16a1 1 0 011-1h4a1 1 0 011 1v3a1 1 0 01-1 1h-4a1 1 0 01-1-1v-3z"></path>
                                            </svg>
                                            "المساحة (متر²)"
                                        </label>
                                        <input
                                            class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-cyan-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                                            type="number"
                                            name="space_in_meters"
                                            id="space_in_meters"
                                            min="0"
                                            placeholder="300"
                                            required
                                        />
                                    </div>

                                    <div class="group">
                                        <label
                                            class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                                            for="price_in_cents"
                                        >
                                            <svg class="w-5 h-5 text-green-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8c-1.657 0-3 .895-3 2s1.343 2 3 2 3 .895 3 2-1.343 2-3 2m0-8c1.11 0 2.08.402 2.599 1M12 8V7m0 1v8m0 0v1m0-1c-1.11 0-2.08-.402-2.599-1M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
                                            </svg>
                                            "السعر (قرش)"
                                        </label>
                                        <input
                                            class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-green-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                                            type="number"
                                            name="price_in_cents"
                                            id="price_in_cents"
                                            min="0"
                                            placeholder="500000"
                                            required
                                        />
                                    </div>
                                </div>

                                <div class="bg-blue-50 border-l-4 border-blue-500 p-4 rounded-lg">
                                    <p class="text-sm text-gray-600">
                                        <span class="font-semibold text-blue-700">"ملحوظة:"</span>
                                        " السعر يُدخل بالقرش (1 جنيه = 100 قرش)"
                                    </p>
                                </div>
                            </div>

                            <div class="flex flex-wrap gap-4 justify-center mt-10">
                                <button
                                    class="group px-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold text-lg rounded-xl shadow-lg hover:shadow-2xl hover:scale-105 active:scale-95 transition-all duration-300 flex items-center gap-3"
                                    type="submit"
                                >
                                    <svg class="w-6 h-6 group-hover:rotate-12 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                                    </svg>
                                    "إضافة العقار"
                                </button>

                                <a
                                    href={move || format!("/dashboard/manageEstates/{}", user_id().unwrap_or("".to_string()))}
                                    class="px-8 py-4 bg-white text-gray-700 font-bold text-lg rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 border-2 border-gray-300 hover:border-gray-400 flex items-center gap-2"
                                >
                                    <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                        <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
                                    </svg>
                                    "إلغاء"
                                </a>
                            </div>
                        </ActionForm>
                    </div>
                </div>
            </div>
        </div>
    }
}
