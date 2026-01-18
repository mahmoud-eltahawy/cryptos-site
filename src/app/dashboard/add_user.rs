use crate::app::Level;
use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

#[server]
async fn add_user(
    id: Uuid,
    name: String,
    level: Level,
    password: String,
) -> Result<(), ServerFnError> {
    use crate::app::DB;

    DB.users.lock().unwrap().push(crate::app::User {
        id,
        name,
        password: password_auth::generate_hash(password),
        level,
    });
    leptos_axum::redirect(&format!("/dashboard/{}", id));
    Ok(())
}

#[component]
pub fn AddUser() -> impl IntoView {
    let params = use_params_map();
    let user_id = move || params.with(|p| p.get("id"));

    let add_user = ServerAction::<AddUser>::new();

    view! {
        <div class="grid grid-cols-1 gap-5 text-center border-5 rounded-lg my-10 mx-5 p-1 md:p-3 lg:p-5">
            <ActionForm action={add_user}>
                <input class="hidden" type="text" value={user_id} name="id"/>
                <div class="grid grid-cols-1 gap-2 my-10">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="name"
                    >"اسم المستخدم"</label>
                    <input
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        type="text"
                        name="name"
                        id="name"
                    />
                </div>
                <div class="text-center grid grid-cols-1 gap-2 my-10">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="level"
                    >"النفوذ"</label>
                    <select
                        name="level"
                        id="level"
                        class="p-5 border-2 border-gray-300 rounded-lg text-center block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                    >
                      <option value="Admin">"ادمين"</option>
                      <option value="User">"مستخدم"</option>
                    </select>
                </div>
                <div class="grid grid-cols-1 gap-2 my-10">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="password"
                    >"كلمة السر"</label>
                    <input
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        type="password"
                        name="password"
                        id="password"
                    />
                </div>
                <input
                    class="w-auto px-4 py-2 text-white bg-blue-600 rounded-md hover:bg-blue-700 focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2"
                    type="submit"
                    value="تأكيد"
                />
            </ActionForm>
        </div>
    }
}
