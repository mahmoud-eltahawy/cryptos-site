use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::app::Estate;
use crate::auth::AuthRequired;

#[server]
async fn get_estate_by_id(id: uuid::Uuid) -> Result<Estate, ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let estate = crate::db::estates::get_estate_by_id(&app_state.pool, id).await;
    let Ok(estate) = estate else {
        return Err(ServerFnError::ServerError(
            "could not find estate with id".to_string(),
        ));
    };
    Ok(estate)
}

#[server]
async fn update_name(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    name: String,
) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_name(&app_state.pool, target_id, name)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!(
        "/dashboard/updateEstate/{}/{}",
        target_id, user_id
    ));
    Ok(())
}

#[server]
async fn update_address(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    address: String,
) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_address(&app_state.pool, target_id, address)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!(
        "/dashboard/updateEstate/{}/{}",
        target_id, user_id
    ));
    Ok(())
}

#[server]
async fn update_image_url(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    image_url: String,
) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_image_url(&app_state.pool, target_id, image_url)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!(
        "/dashboard/updateEstate/{}/{}",
        target_id, user_id
    ));
    Ok(())
}

#[server]
async fn update_description(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    description: String,
) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_description(&app_state.pool, target_id, description)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!(
        "/dashboard/updateEstate/{}/{}",
        target_id, user_id
    ));
    Ok(())
}

#[server]
async fn update_price(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    price_in_cents: i64,
) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_price(&app_state.pool, target_id, price_in_cents)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!(
        "/dashboard/updateEstate/{}/{}",
        target_id, user_id
    ));
    Ok(())
}

#[server]
async fn update_space(
    user_id: uuid::Uuid,
    target_id: uuid::Uuid,
    space_in_meters: i32,
) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_space(&app_state.pool, target_id, space_in_meters)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!(
        "/dashboard/updateEstate/{}/{}",
        target_id, user_id
    ));
    Ok(())
}

#[component]
pub fn UpdateEstate() -> impl IntoView {
    let update_name = ServerAction::<UpdateName>::new();
    let update_address = ServerAction::<UpdateAddress>::new();
    let update_image_url = ServerAction::<UpdateImageUrl>::new();
    let update_description = ServerAction::<UpdateDescription>::new();
    let update_price = ServerAction::<UpdatePrice>::new();
    let update_space = ServerAction::<UpdateSpace>::new();

    let params = use_params_map();
    let target_id = move || params.with(|p| p.get("targetId"));
    let target_res = Resource::new(
        move || {
            target_id()
                .as_ref()
                .and_then(|x| Uuid::parse_str(x).ok())
                .unwrap_or(Uuid::nil())
        },
        get_estate_by_id,
    );

    let user_id = move || params.with(|p| p.get("userId")).unwrap_or(String::new());
    let target = move || target_res.get().and_then(|x| x.ok());

    view! {
        <AuthRequired>
        <Suspense>
        <div class="grid grid-cols-1 gap-5 text-center border-5 rounded-lg my-10 mx-5 p-1 md:p-3 lg:p-5">
            <h1 class="text-2xl font-bold mb-5">"تحديث بيانات العقار"</h1>

            <ActionForm action={update_name}>
                <input class="hidden" type="text" value={user_id} name="user_id"/>
                <input class="hidden" type="text" value={target_id} name="target_id"/>
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
                        value={move || target().map(|x| x.name)}
                    />
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث الاسم"
                    />
                </div>
            </ActionForm>

            <ActionForm action={update_address}>
                <input class="hidden" type="text" value={user_id} name="user_id"/>
                <input class="hidden" type="text" value={target_id} name="target_id"/>
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
                        value={move || target().map(|x| x.address)}
                    />
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث العنوان"
                    />
                </div>
            </ActionForm>

            <ActionForm action={update_image_url}>
                <input class="hidden" type="text" value={user_id} name="user_id"/>
                <input class="hidden" type="text" value={target_id} name="target_id"/>
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
                        value={move || target().map(|x| x.image_url)}
                    />
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث الصورة"
                    />
                </div>
            </ActionForm>

            <ActionForm action={update_description}>
                <input class="hidden" type="text" value={user_id} name="user_id"/>
                <input class="hidden" type="text" value={target_id} name="target_id"/>
                <div class="grid grid-cols-1 gap-2 my-5">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="description"
                    >"الوصف"</label>
                    <textarea
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        name="description"
                        id="description"
                    >{move || target().map(|x| x.description)}</textarea>
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث الوصف"
                    />
                </div>
            </ActionForm>

            <ActionForm action={update_price}>
                <input class="hidden" type="text" value={user_id} name="user_id"/>
                <input class="hidden" type="text" value={target_id} name="target_id"/>
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
                        value={move || target().map(|x| x.price_in_cents.to_string())}
                    />
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث السعر"
                    />
                </div>
            </ActionForm>

            <ActionForm action={update_space}>
                <input class="hidden" type="text" value={user_id} name="user_id"/>
                <input class="hidden" type="text" value={target_id} name="target_id"/>
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
                        value={move || target().map(|x| x.space_in_meters.to_string())}
                    />
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث المساحة"
                    />
                </div>
            </ActionForm>

            <div class="mt-5">
                <a
                    href="/dashboard/manageEstates"
                    class="px-6 py-2 text-gray-700 bg-gray-300 rounded-md hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
                >"العودة إلى إدارة العقارات"</a>
            </div>
        </div>
        </Suspense>
        </AuthRequired>
    }
}
