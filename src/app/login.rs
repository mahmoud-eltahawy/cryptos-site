use leptos::prelude::*;

#[server]
async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    use crate::app::DB;
    let id = DB
        .users
        .iter()
        .find(|x| x.name == username)
        .and_then(|user| {
            password_auth::verify_password(password, &user.password)
                .ok()
                .map(|_| user.id)
        });
    let Some(id) = id else {
        return Err(ServerFnError::Args(
            "username or password is wrong".to_string(),
        ));
    };
    leptos_axum::redirect(&format!("/dashboard/{}", id));
    Ok(())
}

#[component]
pub fn LoginPage() -> impl IntoView {
    let login_ac = ServerAction::<Login>::new();

    view! {
        <div class="grid grid-cols-1 gap-5 text-center border-5 rounded-lg my-10 mx-5 p-1 md:p-3 lg:p-5">
            <ActionForm action={login_ac}>
                <div class="grid grid-cols-1 gap-2 my-10">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="username"
                    >"اسم المستخدم"</label>
                    <input
                        class="text-center w-full px-4 py-2 border-2 border-gray-300 rounded-lg focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-blue-500 md:border-green-400"
                        type="text"
                        name="username"
                        id="username"
                    />
                </div>
                <div class="grid grid-cols-1 gap-2 my-10">
                    <label
                        class="block text-sm font-bold mb-2 sm:text-base lg:text-xl"
                        for="username"
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
