use leptos::prelude::*;

#[server]
async fn login(username: String, password: String) -> Result<(), ServerFnError> {
    use crate::auth::set_user_session;
    use tower_sessions::Session;

    let app_state = use_context::<crate::AppState>()
        .ok_or_else(|| ServerFnError::new("No App State found".to_string()))?;

    let parts = use_context::<axum::http::request::Parts>()
        .ok_or_else(|| ServerFnError::new("No request parts found".to_string()))?;
    let session = parts
        .extensions
        .get::<Session>()
        .ok_or_else(|| ServerFnError::new("No session found".to_string()))?
        .clone();

    let user_error = "اسم المستخدم أو كلمة السر غير صحيحة".to_string();

    let user = crate::db::users::get_user_by_name(&app_state.pool, &username)
        .await
        .map_err(|_| ServerFnError::new(&user_error))?;

    if password_auth::verify_password(&password, &user.password).is_err() {
        return Err(ServerFnError::Args(user_error));
    }

    set_user_session(session, user.id, user.level)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;

    println!("{} loged in", user.name);
    leptos_axum::redirect(&format!("/dashboard/{}", user.id));
    Ok(())
}

#[component]
pub fn Login() -> impl IntoView {
    let login_ac = ServerAction::<Login>::new();
    let error_msg = move || {
        login_ac
            .value()
            .get()
            .and_then(|res| res.err())
            .map(|e| e.to_string())
    };

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

                    <ShowLet some=error_msg let(msg)>
                        <div class="mb-6 bg-red-50 border-l-4 border-red-500 p-4 rounded-lg">
                            <div class="flex items-center gap-2">
                                <RedCircleIcon/>
                                <p class="text-sm text-red-800 font-semibold">{msg}</p>
                            </div>
                        </div>
                    </ShowLet>

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

#[component]
fn RedCircleIcon() -> impl IntoView {
    view! {
        <svg class="w-5 h-5 text-red-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 8v4m0 4h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
    }
}
