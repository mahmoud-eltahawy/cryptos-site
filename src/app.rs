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
pub use crate::models::{Estate, SecureUser};
use features_section::FeaturesSection;
use hero_section::HeroSection;
use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};
use navbar::Navbar;

mod dashboard;
mod features_section;
mod hero_section;
mod login;
mod navbar;

#[cfg(feature = "ssr")]
pub use crate::models::User;

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
                    <Route path=path!("/dashboard/updateUser/:targetId") view=UpdateUser/>
                    <Route path=path!("/dashboard/updateEstate/:targetId/:userId") view=UpdateEstate/>
                    <Route path=path!("/dashboard/estateDetails/:targetId/:userId") view=EstateDetails/>
                    <Route path=path!("/dashboard/addUser") view=AddUser/>
                    <Route path=path!("/dashboard/manageUser") view=ManageUser/>
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
            <HeroSection/>
            <FeaturesSection/>
            <Footer/>
        </div>
    }
}

#[component]
fn Footer() -> impl IntoView {
    view! {
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
    }
}
