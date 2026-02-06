use leptos::prelude::*;
use web_sys::{FormData, HtmlFormElement, HtmlInputElement, SubmitEvent, wasm_bindgen::JsCast};

use crate::auth::AuthRequired;

#[server]
async fn add_estate(
    name: String,
    address: String,
    image_url: String,
    price_in_cents: i64,
    space_in_meters: i32,
    description: String,
) -> Result<(), ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    crate::db::estates::create_estate(
        &app_state.pool,
        name,
        address,
        image_url,
        price_in_cents,
        space_in_meters,
        description,
    )
    .await
    .map_err(|e| ServerFnError::new(e.to_string()))?;
    leptos_axum::redirect("/dashboard/manageEstates");
    Ok(())
}

#[component]
pub fn AddEstate() -> impl IntoView {
    let add_estate = ServerAction::<AddEstate>::new();

    view! {
        <AuthRequired>
            <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 py-12 px-4">
                <div class="max-w-3xl mx-auto">
                    <div class="bg-white/90 backdrop-blur-lg rounded-3xl shadow-2xl overflow-hidden border border-gray-100">
                        <Banner/>
                        <div class="p-8">
                            <ActionForm action={add_estate}>
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
                                            minlength="3"
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
                                            minlength="5"
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

                                    <div class="group">
                                        <label
                                            class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                                            for="description"
                                        >
                                            <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M17.657 16.657L13.414 20.9a1.998 1.998 0 01-2.827 0l-4.244-4.243a8 8 0 1111.314 0z"></path>
                                                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M15 11a3 3 0 11-6 0 3 3 0 016 0z"></path>
                                            </svg>
                                            "الوصف"
                                        </label>
                                        <textarea
                                            class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                                            type="text"
                                            name="description"
                                            id="description"
                                            placeholder="ضع اي معلومات اضافية هنا"
                                            required
                                            minlength="15"
                                        ></textarea>
                                    </div>
                                </div>

                                    <UploadImage/>
                            </ActionForm>
                        </div>
                    </div>
                </div>
            </div>
        </AuthRequired>
    }
}

#[server(input = server_fn::codec::MultipartFormData)]
async fn upload_image(data: server_fn::codec::MultipartData) -> Result<String, ServerFnError> {
    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let mut data = data.into_inner().unwrap();

    let mut image_data = Vec::new();
    let mut image_name = String::new();
    let mut image_kind = String::new();

    while let Ok(Some(mut field)) = data.next_field().await {
        let name = field.name().unwrap_or_default().to_string();
        match name.as_str() {
            "data" => {
                while let Ok(Some(chunk)) = field.chunk().await {
                    image_data.extend(chunk.to_vec());
                }
            }
            "image_name" => {
                if let Ok(imgn) = field.text().await {
                    image_name = imgn;
                }
            }
            "image_kind" => {
                if let Ok(imgk) = field.text().await {
                    image_kind = imgk;
                }
            }
            _ => (),
        };
    }
    if image_data.is_empty() {
        return Err(ServerFnError::new("no data was recieved for the image"));
    } else if image_name.is_empty() {
        return Err(ServerFnError::new("no name was recieved for the image"));
    } else if image_kind.is_empty() {
        return Err(ServerFnError::new("no kind was recieved for the image"));
    }
    let prefix = uuid::Uuid::new_v4().to_string();
    image_name = prefix + &image_name;
    let bucket = "images";

    app_state
        .s3_client
        .put_object()
        .bucket(bucket)
        .key(&image_name)
        .body(image_data.into())
        .content_type(image_kind)
        .send()
        .await?;

    Ok(format!("http://localhost:9000/{bucket}/{image_name}"))
}

#[island]
fn UploadImage() -> impl IntoView {
    let file_input = NodeRef::new();

    let upload_action =
        Action::new_local(|data: &web_sys::FormData| upload_image(data.clone().into()));

    let on_submit = move |ev: SubmitEvent| {
        ev.prevent_default();
        let input: HtmlInputElement = file_input.get().unwrap();
        let files = input.files().unwrap();
        let file = files.get(0).unwrap();
        let name = file.name();
        let kind = file.type_();

        let target = ev.target().unwrap().unchecked_into::<HtmlFormElement>();
        let form_data = FormData::new_with_form(&target).unwrap();
        let _ = form_data.append_with_str("image_name", &name);
        let _ = form_data.append_with_str("image_kind", &kind);
        upload_action.dispatch_local(form_data);
    };

    let image = move || upload_action.value().get().transpose().ok().flatten();

    view! {
    <div class="flex flex-wrap gap-4 justify-center mt-10">
        <ShowLet
            some=image
            let(image_url)
            fallback=move || view!{
                <form
                    id="INNER_FORM"
                    on:submit=on_submit
                    class="grid grid-cols-2 gap-3 shadow-lg pb-2 mb-5"
                >
                    <label
                        class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                        for="data"
                    >
                        <svg class="w-5 h-5 text-pink-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 16l4.586-4.586a2 2 0 012.828 0L16 16m-2-2l1.586-1.586a2 2 0 012.828 0L20 14m-6-6h.01M6 20h12a2 2 0 002-2V6a2 2 0 00-2-2H6a2 2 0 00-2 2v12a2 2 0 002 2z"></path>
                        </svg>
                        "الصورة"
                    </label>
                    <input
                        node_ref={file_input}
                        class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                        type="file"
                        accept=".png, .jpg, .jpeg, .webp"
                        name="data"
                        id="data"
                        required
                    />
                    <button
                        class="group px-8 py-4 bg-gradient-to-r from-blue-500 to-violet-500 text-white font-bold text-lg rounded-xl shadow-lg hover:shadow-2xl hover:scale-105 active:scale-95 transition-all duration-300 flex items-center gap-3"
                        type="submit"
                        form="INNER_FORM"
                    >
                        <svg class="w-6 h-6 group-hover:rotate-12 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                        </svg>
                        "تأكيد الصورة"
                    </button>
                    <CancelButton/>
                </form>
            }
        >
            <input
                class="hidden"
                value={image_url.clone()}
                name="image_url"
                type="text"
            />
            <img
                src={image_url}
                alt="estate image"
            />
            <button
                class="group px-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold text-lg rounded-xl shadow-lg hover:shadow-2xl hover:scale-105 active:scale-95 transition-all duration-300 flex items-center gap-3"
                type="submit"
            >
                <svg class="w-6 h-6 group-hover:rotate-12 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
                </svg>
                "إضافة العقار"
            </button>
            <CancelButton/>
        </ShowLet>
    </div>
    }
}

#[component]
fn CancelButton() -> impl IntoView {
    view! {
        <a
            href="/dashboard/manageEstates"
            class="px-8 py-4 bg-white text-gray-700 font-bold text-lg rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 border-2 border-gray-300 hover:border-gray-400 flex items-center gap-2"
        >
            <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
            </svg>
            "إلغاء"
        </a>
    }
}

#[component]
fn Banner() -> impl IntoView {
    view! {
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
    }
}

#[component]
fn Spinner() -> impl IntoView {
    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
            <div class="text-center">
                <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                <p class="mt-4 text-gray-600">"جاري التحقق من الهوية..."</p>
            </div>
        </div>
    }
}
