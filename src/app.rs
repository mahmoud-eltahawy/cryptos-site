use std::sync::{LazyLock, Mutex};

use crate::app::{
    dashboard::{
        Dashboard,
        manage_estates::{
            ManageEstates, add_estate::AddEstate, estate_details::EstateDetails,
            update_estate::UpdateEstate,
        },
        manage_user::{ManageUser, add_user::AddUser, update_user::UpdateUser},
    },
    login::Login,
};
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};
use navbar::Navbar;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

mod dashboard;
mod login;
mod navbar;

#[derive(Debug, Serialize, Deserialize, Clone)]
enum Level {
    Admin,
    User,
}

struct User {
    id: Uuid,
    name: String,
    password: String,
    level: Level,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Estate {
    id: Uuid,
    name: String,
    address: String,
    image_url: String,
    price_in_cents: usize,
    space_in_meters: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct SecureUser {
    id: Uuid,
    name: String,
    level: Level,
}

impl From<&User> for SecureUser {
    fn from(
        User {
            id,
            name,
            password: _,
            level,
        }: &User,
    ) -> Self {
        Self {
            id: *id,
            name: name.clone(),
            level: level.clone(),
        }
    }
}

struct Db {
    users: Mutex<Vec<User>>,
    estates: Mutex<Vec<Estate>>,
}

impl Db {
    fn new() -> Self {
        Db {
            users: Mutex::new(vec![
                User {
                    id: Uuid::new_v4(),
                    name: String::from("admin"),
                    password: password_auth::generate_hash("admin"),
                    level: Level::Admin,
                },
                User {
                    id: Uuid::new_v4(),
                    name: String::from("احمد"),
                    password: password_auth::generate_hash("ahmed"),
                    level: Level::User,
                },
                User {
                    id: Uuid::new_v4(),
                    name: String::from("مصطفي"),
                    password: password_auth::generate_hash("mostafa"),
                    level: Level::User,
                },
                User {
                    id: Uuid::new_v4(),
                    name: String::from("علي"),
                    password: password_auth::generate_hash("ali"),
                    level: Level::User,
                },
            ]),
            estates: Mutex::new(vec![
                Estate {
                    id: Uuid::new_v4(),
                    name: String::from("شقة 1"),
                    address: String::from("عنوان 1"),
                    image_url: String::from(
                        "data:image/jpeg;base64,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",
                    ),
                    price_in_cents: 4_000_000_00,
                    space_in_meters: 110,
                },
                Estate {
                    id: Uuid::new_v4(),
                    name: String::from("شقة 2"),
                    address: String::from("عنوان 2"),
                    image_url: String::from(
                        "data:image/jpeg;base64,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",
                    ),
                    price_in_cents: 4_000_000_00,
                    space_in_meters: 130,
                },
            ]),
        }
    }
}

static DB: LazyLock<Db> = LazyLock::new(Db::new);

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="ar" dir="rtl">
            <head>
                <AutoReload options=options.clone() />
                <HydrationScripts options islands=true/>
                <MetaTags/>
                <link rel="icon" href="black-logo.png" type="image/png"/>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
            </head>
            <body class="bg-gradient-to-r from-sky-950 to-violet-200 bg-cover text-sm md:text-base lg:text-lg">
                <App/>
            </body>
        </html>
    }
}

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();
    view! {
        <Stylesheet id="leptos" href="/pkg/cryptos-site.css"/>
        <Title text="كريبتوس للتسويق و الاستثمار و التطوير العقاري | cryptos"/>
        <Router>
            <main>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=HomePage/>
                    <Route path=StaticSegment("/login") view=Login/>
                    <Route path=path!("/dashboard/updateUser/:targetId/:userId") view=UpdateUser/>
                    <Route path=path!("/dashboard/updateEstate/:targetId/:userId") view=UpdateEstate/>
                    <Route path=path!("/dashboard/estateDetails/:targetId/:userId") view=EstateDetails/>
                    <Route path=path!("/dashboard/addUser/:id") view=AddUser/>
                    <Route path=path!("/dashboard/manageUser/:id") view=ManageUser/>
                    <Route path=path!("/dashboard/manageEstates/:id") view=ManageEstates/>
                    <Route path=path!("/dashboard/addEstate/:id") view=AddEstate/>
                    <Route path=path!("/dashboard/:id") view=Dashboard/>
                </Routes>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <Navbar/>

            // Hero Section
            <div class=r#"relative bg-gradient-to-br from-blue-600 via-purple-600 to-pink-600 min-h-screen flex items-center justify-center overflow-hidden"#>
                <div class="absolute inset-0 bg-[url('/background.jpg')] bg-cover bg-center opacity-20"></div>
                <div class="absolute inset-0 bg-gradient-to-br from-blue-900/50 to-purple-900/50"></div>

                // Animated circles
                <div class="absolute top-20 left-20 w-72 h-72 bg-blue-400/30 rounded-full blur-3xl animate-pulse"></div>
                <div class="absolute bottom-20 right-20 w-96 h-96 bg-purple-400/30 rounded-full blur-3xl animate-pulse delay-700"></div>

                <div class="relative z-10 max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <div class="animate-fade-in">
                        <h1 class="text-5xl md:text-7xl font-bold text-white mb-6 leading-tight">
                            "كريبتوس للتسويق"
                            <br/>
                            <span class="bg-gradient-to-r from-yellow-300 to-pink-300 bg-clip-text text-transparent">
                                "والاستثمار العقاري"
                            </span>
                        </h1>

                        <p class="text-xl md:text-2xl text-blue-100 mb-12 max-w-3xl mx-auto leading-relaxed">
                            "نساعدك في العثور على العقار المثالي واستثمار أموالك بذكاء"
                        </p>

                        <div class="flex flex-wrap gap-6 justify-center">
                            <a
                                href="#estates"
                                class="group px-8 py-4 bg-white text-blue-600 font-bold text-lg rounded-full shadow-2xl hover:shadow-white/20 hover:scale-110 transition-all duration-300 flex items-center gap-3"
                            >
                                "استكشف العقارات"
                                <svg class="w-6 h-6 group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"></path>
                                </svg>
                            </a>

                            <a
                                href="/login"
                                class="px-8 py-4 bg-transparent border-2 border-white text-white font-bold text-lg rounded-full hover:bg-white hover:text-blue-600 shadow-xl hover:shadow-2xl hover:scale-110 transition-all duration-300"
                            >
                                "تسجيل الدخول"
                            </a>
                        </div>
                    </div>

                    // Stats
                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8 mt-20">
                        <div class="bg-white/10 backdrop-blur-md rounded-2xl p-6 border border-white/20 hover:scale-105 transition-transform duration-300">
                            <div class="text-4xl font-bold text-white mb-2">"500+"</div>
                            <div class="text-blue-100">"عقار متاح"</div>
                        </div>
                        <div class="bg-white/10 backdrop-blur-md rounded-2xl p-6 border border-white/20 hover:scale-105 transition-transform duration-300">
                            <div class="text-4xl font-bold text-white mb-2">"1000+"</div>
                            <div class="text-blue-100">"عميل سعيد"</div>
                        </div>
                        <div class="bg-white/10 backdrop-blur-md rounded-2xl p-6 border border-white/20 hover:scale-105 transition-transform duration-300">
                            <div class="text-4xl font-bold text-white mb-2">"15+"</div>
                            <div class="text-blue-100">"سنة خبرة"</div>
                        </div>
                    </div>
                </div>
            </div>

            // Features Section
            <div id="estates" class="py-20 bg-gradient-to-br from-gray-50 to-blue-50">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                    <div class="text-center mb-16">
                        <h2 class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4">
                            "لماذا تختار كريبتوس؟"
                        </h2>
                        <p class="text-gray-600 text-xl">"نقدم لك أفضل الحلول العقارية"</p>
                    </div>

                    <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                        <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105 border border-gray-100">
                            <div class="bg-gradient-to-br from-blue-500 to-cyan-500 w-16 h-16 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300">
                                <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-800 mb-4">"عقارات متنوعة"</h3>
                            <p class="text-gray-600 leading-relaxed">"مجموعة واسعة من العقارات السكنية والتجارية التي تناسب جميع الاحتياجات والميزانيات"</p>
                        </div>

                        <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105 border border-gray-100">
                            <div class="bg-gradient-to-br from-purple-500 to-pink-500 w-16 h-16 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300">
                                <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-800 mb-4">"موثوقية عالية"</h3>
                            <p class="text-gray-600 leading-relaxed">"نضمن لك التعامل الآمن والشفاف مع فريق محترف من الخبراء في المجال العقاري"</p>
                        </div>

                        <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105 border border-gray-100">
                            <div class="bg-gradient-to-br from-green-500 to-emerald-500 w-16 h-16 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300">
                                <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 10V3L4 14h7v7l9-11h-7z"></path>
                                </svg>
                            </div>
                            <h3 class="text-2xl font-bold text-gray-800 mb-4">"خدمة سريعة"</h3>
                            <p class="text-gray-600 leading-relaxed">"نساعدك في إيجاد العقار المناسب بسرعة وكفاءة مع دعم فني على مدار الساعة"</p>
                        </div>
                    </div>
                </div>
            </div>

            // Footer
            <footer class="bg-gradient-to-r from-gray-900 to-gray-800 text-white py-12">
                <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                    <div class="flex items-center justify-center gap-3 mb-4">
                        <div class="bg-gradient-to-br from-blue-600 to-purple-600 p-3 rounded-xl">
                            <img width="40" height="40" src="black-logo.png" alt="logo" class="brightness-0 invert"/>
                        </div>
                        <span class="text-2xl font-bold">"Cryptos"</span>
                    </div>
                    <p class="text-gray-400 mb-2">"كريبتوس للتسويق والاستثمار والتطوير العقاري"</p>
                    <p class="text-gray-500 text-sm">"© 2024 جميع الحقوق محفوظة"</p>
                </div>
            </footer>
        </div>
    }
}
