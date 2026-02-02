use leptos::prelude::*;
use leptos_router::hooks::use_params_map;

use crate::app::Estate;

pub mod add_estate;
pub mod update_estate;

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
            let(Estate { id, name, address, image_url, price_in_cents })
        >
            <ActionForm action={remove_estate}>
                <div class="grid grid-cols-5 gap-5 text-center m-5">
                    <input class="hidden" name="id" value={user_id}/>
                    <input class="hidden" name="target_id" value={id.to_string()}/>
                    <h3 class="text-xl col-span-3">{name}</h3>
                    <input class="text-red-800 hover:text-red-400 border-2 rounded-lg col-span-1" type="submit" value="حذف"/>
                    <a
                        class="text-lime-800 hover:text-lime-400 border-2 rounded-lg col-span-1"
                        href={move || format!("/dashboard/updateEstate/{}/{}",id,user_id().unwrap_or("".to_string()))}
                    >"تحديث"</a>
                </div>
            </ActionForm>
        </For>
        </Suspense>
        <div class="grid grid-cols-1 text-center">
            <a
                href={move || format!("/dashboard/addEstate/{}",user_id().unwrap_or("".to_string()))}
                class="text-violet-800 hover:text-violet-500 border-2 rounded-lg w-5/6 m-5 p-2 text-xl hover:text-2xl"
            >"+ اضافة +"</a>
        </div>
    }
}
