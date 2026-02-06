use std::path::PathBuf;

use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;
use web_sys::wasm_bindgen::JsCast;
use web_sys::{FormData, HtmlFormElement};

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
async fn update_name(target_id: uuid::Uuid, name: String) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_name(&app_state.pool, target_id, name)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateEstate/{}", target_id));
    Ok(())
}

#[server]
async fn update_address(target_id: uuid::Uuid, address: String) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_address(&app_state.pool, target_id, address)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateEstate/{}", target_id));
    Ok(())
}

#[server(input = server_fn::codec::MultipartFormData)]
async fn update_image_url(data: server_fn::codec::MultipartData) -> Result<String, ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let mut data = data.into_inner().unwrap();

    let mut image_data = Vec::new();
    let mut target_id = Uuid::nil();
    let mut image_url = String::new();

    while let Ok(Some(mut field)) = data.next_field().await {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "data" => {
                while let Ok(Some(data)) = field.chunk().await {
                    image_data.extend(data.to_vec());
                }
            }
            "image_url" => {
                if let Ok(iu) = field.text().await {
                    image_url = iu;
                }
            }
            "target_id" => {
                if let Ok(ti) = field.text().await {
                    target_id = Uuid::parse_str(&ti)?;
                }
            }
            _ => (),
        };
    }
    if image_data.is_empty() {
        return Err(ServerFnError::new("no data was recieved for the image"));
    }
    if image_url.is_empty() {
        return Err(ServerFnError::new("no url was recieved for the image"));
    }
    if target_id.is_nil() {
        return Err(ServerFnError::new("no id was recieved for the image"));
    }

    let image_name = PathBuf::from(image_url)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    let url = app_state
        .s3
        .store_image(&image_name, None, image_data)
        .await?;

    Ok(format!("{}?vv={}", url, Uuid::new_v4()))
}

#[server]
async fn update_description(
    target_id: uuid::Uuid,
    description: String,
) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_description(&app_state.pool, target_id, description)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateEstate/{}", target_id));
    Ok(())
}

#[server]
async fn update_price(target_id: uuid::Uuid, price_in_cents: i64) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_price(&app_state.pool, target_id, price_in_cents)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateEstate/{}", target_id));
    Ok(())
}

#[server]
async fn update_space(target_id: uuid::Uuid, space_in_meters: i32) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::update_estate_space(&app_state.pool, target_id, space_in_meters)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    leptos_axum::redirect(&format!("/dashboard/updateEstate/{}", target_id));
    Ok(())
}

#[component]
pub fn UpdateEstate() -> impl IntoView {
    let update_name = ServerAction::<UpdateName>::new();
    let update_address = ServerAction::<UpdateAddress>::new();
    let update_description = ServerAction::<UpdateDescription>::new();
    let update_price = ServerAction::<UpdatePrice>::new();
    let update_space = ServerAction::<UpdateSpace>::new();

    let params = use_params_map();
    let target_id = move || {
        params
            .with(|p| p.get("targetId"))
            .and_then(|x| Uuid::parse_str(&x).ok())
            .unwrap_or(Uuid::nil())
    };
    let str_target_id = move || target_id().to_string();
    let target_res = Resource::new(target_id, get_estate_by_id);

    let target = move || target_res.get().and_then(|x| x.ok());

    view! {
        <AuthRequired>
        <Suspense>
        <ShowLet
            some=target
            let(Estate{id,image_url,address,name,price_in_cents,space_in_meters,description})
        >
        <div class="grid grid-cols-1 gap-5 text-center border-5 rounded-lg my-10 mx-5 p-1 md:p-3 lg:p-5">
            <h1 class="text-2xl font-bold mb-5">"تحديث بيانات العقار"</h1>

            <ActionForm action={update_name}>
                <input class="hidden" type="text" value={str_target_id} name="target_id"/>
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
                        value={name}
                    />
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث الاسم"
                    />
                </div>
            </ActionForm>

            <ActionForm action={update_address}>
                <input class="hidden" type="text" value={str_target_id} name="target_id"/>
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
                        value={address}
                    />
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث العنوان"
                    />
                </div>
            </ActionForm>
            <UpdateImage target_id={id} image_url={image_url}/>

            <ActionForm action={update_description}>
                <input class="hidden" type="text" value={str_target_id} name="target_id"/>
                <div class="grid grid-cols-1 gap-2 my-5">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="description"
                    >"الوصف"</label>
                    <textarea
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        name="description"
                        id="description"
                    >{description}</textarea>
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث الوصف"
                    />
                </div>
            </ActionForm>

            <ActionForm action={update_price}>
                <input class="hidden" type="text" value={str_target_id} name="target_id"/>
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
                        value={price_in_cents}
                    />
                    <input
                        class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                        type="submit"
                        value="تحديث السعر"
                    />
                </div>
            </ActionForm>

            <ActionForm action={update_space}>
                <input class="hidden" type="text" value={str_target_id} name="target_id"/>
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
                        value={space_in_meters}
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
        </ShowLet>
        </Suspense>
        </AuthRequired>
    }
}

#[island]
fn UpdateImage(target_id: uuid::Uuid, image_url: String) -> impl IntoView {
    let url = RwSignal::new(image_url.clone());

    let action =
        Action::new_local(|data: &web_sys::FormData| update_image_url(data.clone().into()));

    Effect::new(move || {
        let nv = action.value().get().transpose().ok().flatten();
        if let Some(nv) = nv {
            url.set(nv);
        }
    });

    let on_submit = move |ev: web_sys::SubmitEvent| {
        ev.prevent_default();
        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let form_data = FormData::new_with_form(&target).unwrap();
        action.dispatch_local(form_data);
    };
    view! {
        <form on:submit={on_submit}>
            <input class="hidden" type="text" value={target_id.to_string()} name="target_id"/>
            <input class="hidden" type="text" value={image_url.to_string()} name="image_url"/>
            <div class="grid grid-cols-1 gap-2 my-5 place-items-center gap-5">
                <label
                    class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                    for="image_url"
                >"الصورة"</label>
                <div class="grid grid-cols-2 gap-5">
                    <img src={move || url.get()}/>
                    <div class="grid grid-cols-1 gap-5">
                        <input
                            class="px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                            type="file"
                            accept=".png, .jpg, .jpeg, .webp"
                            name="data"
                            id="data"
                            required
                        />
                        <input
                            class="px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                            type="submit"
                            value="تحديث الصورة"
                        />
                    </div>
                </div>
            </div>
        </form>
    }
}
