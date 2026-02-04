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

mod dashboard;
mod login;
mod navbar;

// Re-export database models for use in the app
#[cfg(feature = "ssr")]
pub use crate::models::{Estate, Level, SecureUser, User};

// Client-side versions (without DB functionality)
#[cfg(not(feature = "ssr"))]
pub use crate::models::{Estate, Level, SecureUser};


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
