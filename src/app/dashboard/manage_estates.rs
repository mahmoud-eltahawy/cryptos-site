use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::app::Estate;

pub mod add_estate;
pub mod update_estate;
pub mod estate_details {
    use leptos::prelude::*;
    use leptos_router::hooks::use_params_map;
    use uuid::Uuid;

    use crate::app::Estate;

    #[server]
    async fn get_estate_by_id(id: uuid::Uuid) -> Result<Estate, ServerFnError> {
        let estate = crate::app::DB
            .estates
            .lock()
            .unwrap()
            .iter()
            .find(|x| x.id == id)
            .cloned();
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
            <Suspense fallback=|| view! { <p>"جاري التحميل..."</p> }>
                {move || estate().map(|Estate { id, name, address, image_url, price_in_cents, space_in_meters }| {
                    view! {
                        <div class="max-w-4xl mx-auto my-10 p-5">
                            <div class="bg-white rounded-lg shadow-lg overflow-hidden">
                                <img
                                    class="w-full h-96 object-cover"
                                    src={image_url.clone()}
                                    alt={name.clone()}
                                />
                                <div class="p-8">
                                    <h1 class="text-3xl font-bold text-gray-800 mb-4">{name.clone()}</h1>

                                    <div class="grid grid-cols-1 md:grid-cols-2 gap-6 my-8">
                                        <div class="bg-blue-50 p-4 rounded-lg">
                                            <h3 class="text-lg font-semibold text-blue-800 mb-2">"المساحة"</h3>
                                            <p class="text-2xl text-gray-700">{space_in_meters}" متر مربع"</p>
                                        </div>

                                        <div class="bg-green-50 p-4 rounded-lg">
                                            <h3 class="text-lg font-semibold text-green-800 mb-2">"السعر"</h3>
                                            <p class="text-2xl text-gray-700">{price_in_cents as f32 / 100.0}" جنيه"</p>
                                        </div>
                                    </div>

                                    <div class="bg-gray-50 p-4 rounded-lg my-6">
                                        <h3 class="text-lg font-semibold text-gray-800 mb-2">"العنوان"</h3>
                                        <p class="text-xl text-gray-700">{address.clone()}</p>
                                    </div>

                                    <div class="bg-purple-50 p-4 rounded-lg my-6">
                                        <h3 class="text-lg font-semibold text-purple-800 mb-2">"معرف العقار"</h3>
                                        <p class="text-sm text-gray-600 font-mono">{id.to_string()}</p>
                                    </div>

                                    <div class="flex gap-4 justify-center mt-8">
                                        <a
                                            href={format!("/dashboard/updateEstate/{}/{}", id, user_id())}
                                            class="px-6 py-3 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                                        >"تعديل العقار"</a>
                                        <a
                                            href={format!("/dashboard/manageEstates/{}", user_id())}
                                            class="px-6 py-3 text-gray-700 bg-gray-300 rounded-md hover:bg-gray-400 focus:outline-none focus:ring-2 focus:ring-gray-500 focus:ring-offset-2"
                                        >"العودة"</a>
                                    </div>
                                </div>
                            </div>
                        </div>
                    }
                })}
            </Suspense>
        }
    }
}

#[server]
async fn remove_estate(id: uuid::Uuid, target_id: uuid::Uuid) -> Result<(), ServerFnError> {
    crate::app::DB
        .estates
        .lock()
        .unwrap()
        .retain(|x| x.id != target_id);
    leptos_axum::redirect(&format!("/dashboard/manageEstates/{}", id));
    Ok(())
}

#[server]
async fn get_estates() -> Result<Vec<Estate>, ServerFnError> {
    let res = crate::app::DB.estates.lock().unwrap().to_vec();
    Ok(res)
}

#[component]
pub fn ManageEstates() -> impl IntoView {
    let estates_res = Resource::new(|| (), move |_| get_estates());
    let estates = move || estates_res.get().and_then(|x| x.ok()).unwrap_or_default();
    let remove_estate = ServerAction::<RemoveEstate>::new();

    let params = use_params_map();
    let user_id = move || params.with(|p| p.get("id"));

    view! {
        <Suspense>
        <For
            each={estates}
            key=|x| x.id
            let(Estate { id, name, address, image_url, price_in_cents,space_in_meters })
        >
            <ActionForm action={remove_estate}>
                <div class="grid grid-cols-8 gap-5 text-center m-5">
                    <input class="hidden" name="id" value={user_id}/>
                    <input class="hidden" name="target_id" value={id.to_string()}/>
                    <div class="h-64 text-blue-600 font-bold grid grid-cols-2 col-span-7 text-xl border-2 rounded-lg">
                        <img
                             class="w-96 h-64 object-fill"
                            src={image_url} alt="view image"
                        />
                        <div class="text-center text-black">
                            <h2>{name}</h2>
                            <ul class="m-5">
                                <li>{space_in_meters} متر</li>
                                <li>{price_in_cents as f32 / 100.0} جنية</li>
                                <li>{address}</li>
                            </ul>
                            <a
                                class="border-1 rounded-lg p-2 m-6 hover:text-lime-600"
                                href={move || format!("/dashboard/estateDetails/{}/{}",id,user_id().unwrap_or("".to_string()))}
                            >"التفاصيل"</a>
                        </div>
                    </div>
                    <div class="text-center grid grid-cols-1 gap-4">
                        <a
                            class="text-center text-lime-800 hover:text-lime-400 border-2 rounded-lg align-middle"
                            href={move || format!("/dashboard/updateEstate/{}/{}",id,user_id().unwrap_or("".to_string()))}
                        >"^ تحديث ^"</a>
                        <input class="text-red-800 hover:text-red-500 border-2 rounded-lg" type="submit" value="- حذف -"/>
                    </div>
                </div>
            </ActionForm>
        </For>
        </Suspense>
        <div class="grid grid-cols-1 text-center w-full">
            <a
                href={move || format!("/dashboard/addEstate/{}",user_id().unwrap_or("".to_string()))}
                class="text-violet-800 hover:text-violet-500 border-2 rounded-lg w-5/6 m-5 p-2 text-xl hover:text-2xl"
            >"+ اضافة +"</a>
        </div>
    }
}
