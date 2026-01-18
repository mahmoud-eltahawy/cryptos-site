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
        <div class="border-5 rounded-lg m-20 p-5 text-center">
            <ActionForm action={login_ac}>
                <div class="grid grid-cols-1">
                    <label class="text-2xl">"اسم المستخدم"</label>
                    <input
                        class="text-center my-5 mx-20 p-5 border-2 rounded-lg"
                        type="text"
                        name="username"
                    />
                </div>
                <div class="grid grid-cols-1">
                    <label class="text-2xl">"كلمة السر"</label>
                    <input
                        class="text-center my-5 mx-20 p-5 border-2 rounded-lg"
                        type="password"
                        name="password"
                    />
                </div>
                <input
                    class="text-center my-5 mx-20 p-5 border-2 rounded-lg hover:text-3xl hover:bg-green-600 hover:cursor-pointer text-xl disabled:text-sm disabled:text-slate-300"
                    type="submit"
                    value="تأكيد"
                />
            </ActionForm>
        </div>
    }
}
