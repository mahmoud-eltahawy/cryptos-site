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
        <div class="grid grid-cols-1 gap-5 text-center border-5 rounded-lg my-10 mx-5 p-1 md:p-3 lg:p-5">
            <h1 class="text-2xl font-bold mb-5">"إضافة عقار جديد"</h1>
            <ActionForm action={add_estate}>
                <input class="hidden" type="text" value={user_id} name="id"/>

                <div class="grid grid-cols-1 gap-2 my-5">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="name"
                    >"اسم العقار"</label>
                    <input
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        type="text"
                        name="name"
                        id="name"
                        required
                    />
                </div>

                <div class="grid grid-cols-1 gap-2 my-5">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="address"
                    >"العنوان"</label>
                    <input
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        type="text"
                        name="address"
                        id="address"
                        required
                    />
                </div>

                <div class="grid grid-cols-1 gap-2 my-5">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="image_url"
                    >"رابط الصورة"</label>
                    <input
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        type="url"
                        name="image_url"
                        id="image_url"
                        required
                    />
                </div>

                <div class="grid grid-cols-1 gap-2 my-5">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="price_in_cents"
                    >"السعر (بالقرش)"</label>
                    <input
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        type="number"
                        name="price_in_cents"
                        id="price_in_cents"
                        min="0"
                        required
                    />
                </div>

                <div class="grid grid-cols-1 gap-2 my-5">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="space_in_meters"
                    >"المساحة (بالمتر)"</label>
                    <input
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        type="number"
                        name="space_in_meters"
                        id="space_in_meters"
                        min="0"
                        required
                    />
                </div>

                <div class="flex gap-4 justify-center mt-8">
                    <input
                        class="px-6 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 cursor-pointer"
                        type="submit"
                        value="تأكيد"
                    />
                    <a
                        href={move || format!("/dashboard/manageEstates/{}", user_id().unwrap_or("".to_string()))}
                        class="px-6 py-2 text-gray-700 bg-gray-300 rounded-md hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
                    >"إلغاء"</a>
                </div>
            </ActionForm>
        </div>
    }
}
