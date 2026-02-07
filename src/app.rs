use crate::app::{
    dashboard::{
        Dashboard,
        manage_estates::{
            ManageEstates, add_estate::AddEstate, estate_details::EstateDetails,
            public_estates::PublicEstates, update_estate::UpdateEstate,
        },
        manage_user::{ManageUser, add_user::AddUser, update_user::UpdateUser},
    },
    login::Login,
    navbar::Footer,
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
                <link rel="icon" href="/black-logo.png" type="image/png"/>
                <meta charset="utf-8"/>
                <meta name="viewport" content="width=device-width, initial-scale=1"/>
            </head>
            <body class="bg-gradient-to-r bg-cover text-sm md:text-base lg:text-lg">
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
                <Navbar/>
                <Routes fallback=|| "Page not found.".into_view()>
                    <Route path=StaticSegment("/") view=HomePage/>
                    <Route path=StaticSegment("/login") view=Login/>
                    <Route path=StaticSegment("/estates") view=PublicEstates/>
                    <Route path=path!("/dashboard/updateUser/:targetId") view=UpdateUser/>
                    <Route path=path!("/dashboard/updateEstate/:targetId") view=UpdateEstate/>
                    <Route path=path!("/dashboard/estateDetails/:targetId") view=EstateDetails/>
                    <Route path=path!("/dashboard/addUser") view=AddUser/>
                    <Route path=path!("/dashboard/manageUser") view=ManageUser/>
                    <Route path=path!("/dashboard/manageEstates") view=ManageEstates/>
                    <Route path=path!("/dashboard/addEstate") view=AddEstate/>
                    <Route path=path!("/dashboard") view=Dashboard/>
                </Routes>
                <Footer/>
            </main>
        </Router>
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class="min-h-screen">
            <HeroSection/>
            <FeaturesSection/>
        </div>
    }
}
