use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

use crate::app::dashboard::get_users_names;

#[server]
async fn remove_user(id: Uuid, target_id: Uuid) -> Result<(), ServerFnError> {
    crate::app::DB
        .users
        .lock()
        .unwrap()
        .retain(|x| x.id != target_id);
    leptos_axum::redirect(&format!("/dashboard/manageUser/{}", id));
    Ok(())
}

#[component]
pub fn ManageUser() -> impl IntoView {
    let users_res = Resource::new(|| (), move |_| get_users_names());
    let users = move || users_res.get().and_then(|x| x.ok()).unwrap_or_default();
    let remove_user = ServerAction::<RemoveUser>::new();

    let params = use_params_map();
    let user_id = move || params.with(|p| p.get("id"));

    view! {
        <Suspense>
        <For
            each={users}
            key=|x| x.0
            let((target_id,name))
        >
            <ActionForm action={remove_user}>
                <div class="grid grid-cols-5 gap-5 text-center m-5">
                    <input class="hidden w-1 h-1" name="target_id" value={target_id.to_string()}/>
                    <input class="hidden w-1 h-1" name="id" value={user_id}/>
                    <h3 class="text-xl col-span-3">{name}</h3>
                    <input class="text-red-800 hover:text-red-400 border-2 rounded-lg col-span-1" type="submit" value="حذف"/>
                    <a
                        class="text-lime-800 hover:text-lime-400 border-2 rounded-lg col-span-1"
                        href={move || format!("/dashboard/updateUser/{}/{}",target_id,user_id().unwrap_or("".to_string()))}
                    >"تحديث"</a>
                </div>
            </ActionForm>
        </For>
        </Suspense>
    }
}
