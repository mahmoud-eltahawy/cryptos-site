use leptos::prelude::*;
use leptos_router::components::Redirect;

use crate::auth::{Level, check_auth};

#[server]
async fn add_user(name: String, level: Level, password: String) -> Result<(), ServerFnError> {
    let pool = use_context::<sqlx::PgPool>()
        .ok_or_else(|| ServerFnError::new("No database pool".to_string()))?;

    let hashed = password_auth::generate_hash(password);
    crate::db::users::create_user(&pool, name, hashed, level)
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?;
    leptos_axum::redirect("/dashboard/manageUser");
    Ok(())
}

#[component]
pub fn AddUser() -> impl IntoView {
    let auth_check = Resource::new(|| (), |_| check_auth());
    let add_user = ServerAction::<AddUser>::new();

    let autherized = move || auth_check.get().map(|x| x.is_ok()).unwrap_or(true);

    #[component]
    pub fn Spinner() -> impl IntoView {
        view! {
            <div class="min-h-screen flex items-center justify-center bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50">
                <div class="text-center">
                    <div class="inline-block animate-spin rounded-full h-12 w-12 border-b-2 border-blue-600"></div>
                    <p class="mt-4 text-gray-600">"جاري التحقق من الهوية..."</p>
                </div>
            </div>
        }
    }

    view! {
        <Suspense fallback=Spinner>
        <Show
            when=autherized
            fallback=move || view! {
                <Redirect path="/login"/>
            }
        >
            <div class="min-h-screen bg-gradient-to-br from-blue-50 via-purple-50 to-pink-50 py-12 px-4">
                <div class="max-w-2xl mx-auto">
                    <div class="bg-white/90 backdrop-blur-lg rounded-3xl shadow-2xl overflow-hidden border border-gray-100">
                        <Banner/>
                        <div class="p-8">
                            <ActionForm action={add_user}>
                                <div class="space-y-6">
                                    <UserNameInput/>
                                    <UserLevelInput/>
                                    <UserPasswordInput/>
                                    <Warn/>
                                </div>
                                <Buttons/>
                            </ActionForm>
                        </div>
                    </div>
                </div>
            </div>
        </Show>
        </Suspense>
    }
}

#[component]
pub fn UserPasswordInput() -> impl IntoView {
    view! {
        <div class="group">
            <label
                class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                for="password"
            >
                <LockIcon/>
                "كلمة السر"
            </label>
            <input
                class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-pink-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                type="password"
                name="password"
                id="password"
                placeholder="أدخل كلمة سر قوية"
                required
                minlength="8"
            />
        </div>
    }
}

#[component]
pub fn UserLevelInput() -> impl IntoView {
    view! {
        <div class="group">
            <label
                class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                for="level"
            >
                <AccessLevelIcon/>
                "مستوى الصلاحية"
            </label>
            <select
                name="level"
                id="level"
                class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-purple-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 font-semibold cursor-pointer"
            >
                <option value="Admin">"مدير (Admin)"</option>
                <option value="User">"مستخدم (User)"</option>
            </select>
        </div>
    }
}

#[component]
pub fn UserNameInput() -> impl IntoView {
    view! {
        <div class="group">
            <label
                class="block text-gray-700 font-bold mb-3 text-lg flex items-center gap-2"
                for="name"
            >
                <UserIcon/>
                "اسم المستخدم"
            </label>
            <input
                class="w-full px-5 py-4 bg-gray-50 border-2 border-gray-200 rounded-xl focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent focus:bg-white transition-all duration-300 text-gray-800 placeholder-gray-400"
                type="text"
                name="name"
                id="name"
                placeholder="أدخل اسم المستخدم"
                required
                minlength="3"
            />
        </div>
    }
}

#[component]
pub fn Warn() -> impl IntoView {
    view! {
        <div class="bg-yellow-50 border-l-4 border-yellow-500 p-4 rounded-lg">
            <div class="flex items-center gap-2">
                <WarnIcon/>
                <p class="text-sm text-yellow-800">
                    <span class="font-semibold">"تنبيه:"</span>
                    " تأكد من استخدام كلمة سر قوية وآمنة"
                </p>
            </div>
        </div>
    }
}

#[component]
pub fn Buttons() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-4 justify-center mt-10">
            <button
                class="group px-8 py-4 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-bold text-lg rounded-xl shadow-lg hover:shadow-2xl hover:scale-105 active:scale-95 transition-all duration-300 flex items-center gap-3"
                type="submit"
            >
                <CheckIcon/>
                "إضافة المستخدم"
            </button>

            <a
                href={move || format!("/dashboard/manageUser")}
                class="px-8 py-4 bg-white text-gray-700 font-bold text-lg rounded-xl shadow-lg hover:shadow-xl hover:scale-105 transition-all duration-300 border-2 border-gray-300 hover:border-gray-400 flex items-center gap-2"
            >
                <CloseIcon/>
                "إلغاء"
            </a>
        </div>
    }
}

#[component]
pub fn UserIcon() -> impl IntoView {
    view! {
        <svg class="w-5 h-5 text-blue-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M16 7a4 4 0 11-8 0 4 4 0 018 0zM12 14a7 7 0 00-7 7h14a7 7 0 00-7-7z"></path>
        </svg>
    }
}

#[component]
pub fn Banner() -> impl IntoView {
    view! {
        <div class="bg-gradient-to-r from-blue-600 to-purple-600 p-8 text-white">
            <div class="flex items-center gap-4">
                <div class="bg-white/20 backdrop-blur-sm p-4 rounded-2xl">
                    <AddUserIcon/>
                </div>
                <div>
                    <h1 class="text-3xl font-bold">"إضافة مستخدم جديد"</h1>
                    <p class="text-blue-100 mt-1">"أدخل بيانات المستخدم الجديد"</p>
                </div>
            </div>
        </div>
    }
}

#[component]
pub fn CloseIcon() -> impl IntoView {
    view! {
        <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M6 18L18 6M6 6l12 12"></path>
        </svg>
    }
}

#[component]
pub fn CheckIcon() -> impl IntoView {
    view! {
        <svg class="w-6 h-6 group-hover:rotate-12 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 13l4 4L19 7"></path>
        </svg>
    }
}

#[component]
pub fn AddUserIcon() -> impl IntoView {
    view! {
        <svg class="w-8 h-8" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M18 9v3m0 0v3m0-3h3m-3 0h-3m-2-5a4 4 0 11-8 0 4 4 0 018 0zM3 20a6 6 0 0112 0v1H3v-1z"></path>
        </svg>
    }
}

#[component]
pub fn AccessLevelIcon() -> impl IntoView {
    view! {
        <svg class="w-5 h-5 text-purple-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
        </svg>
    }
}

#[component]
pub fn LockIcon() -> impl IntoView {
    view! {
        <svg class="w-5 h-5 text-pink-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 15v2m-6 4h12a2 2 0 002-2v-6a2 2 0 00-2-2H6a2 2 0 00-2 2v6a2 2 0 002 2zm10-10V7a4 4 0 00-8 0v4h8z"></path>
        </svg>
    }
}

#[component]
pub fn WarnIcon() -> impl IntoView {
    view! {
        <svg class="w-5 h-5 text-yellow-600" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z"></path>
        </svg>
    }
}
