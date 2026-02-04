use leptos::prelude::*;

#[server]
async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    use crate::app::DB;
    let id = DB
        .users
        .lock()
        .unwrap()
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
pub fn Login() -> impl IntoView {
    let login_ac = ServerAction::<Login>::new();

    view! {
        <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 p-4">
            <div class="w-full max-w-md">
                <div class="bg-white/80 backdrop-blur-xl rounded-2xl shadow-2xl p-8 border border-white/20">
                    <div class="text-center mb-8">
                        <div class="inline-block bg-gradient-to-br from-blue-600 to-purple-600 p-4 rounded-2xl shadow-lg mb-4">
                            <img width="60" height="60" src="black-logo.png" alt="logo" class="brightness-0 invert"/>
                        </div>
                        <h1 class="text-3xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-2">
                            "تسجيل الدخول"
                        </h1>
                        <p class="text-gray-600">"مرحباً بك في كريبتوس"</p>
                    </div>

                    <ActionForm action={login_ac}>
                        <div class="space-y-6">
                            <div>
                                <label
                                    class="block text-gray-700 font-semibold mb-2 text-lg"
                                    for="username"
                                >"اسم المستخدم"</label>
                                <input
                                    class="w-full px-4 py-3 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-300 text-gray-800 placeholder-gray-400"
                                    type="text"
                                    name="username"
                                    id="username"
                                    placeholder="أدخل اسم المستخدم"
                                    required
                                />
                            </div>

                            <div>
                                <label
                                    class="block text-gray-700 font-semibold mb-2 text-lg"
                                    for="password"
                                >"كلمة السر"</label>
                                <input
                                    class="w-full px-4 py-3 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent transition-all duration-300 text-gray-800 placeholder-gray-400"
                                    type="password"
                                    name="password"
                                    id="password"
                                    placeholder="أدخل كلمة السر"
                                    required
                                />
                            </div>

                            <button
                                class="w-full py-3 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold rounded-xl shadow-lg hover:shadow-xl hover:scale-[1.02] active:scale-[0.98] transition-all duration-300"
                                type="submit"
                            >
                                "تسجيل الدخول"
                            </button>
                        </div>
                    </ActionForm>

                    <div class="mt-6 text-center">
                        <a
                            href="/"
                            class="text-gray-600 hover:text-blue-600 transition-colors duration-300 font-medium"
                        >
                            "← العودة إلى الصفحة الرئيسية"
                        </a>
                    </div>
                </div>

                <div class="mt-8 text-center">
                    <p class="text-gray-600 text-sm">
                        "كريبتوس للتسويق و الاستثمار و التطوير العقاري"
                    </p>
                </div>
            </div>
        </div>
    }
}
