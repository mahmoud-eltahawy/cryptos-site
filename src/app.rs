use std::sync::LazyLock;

use leptos::prelude::*;
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};
use navbar::Navbar;
use uuid::Uuid;

use crate::app::login::LoginPage;

mod login;
mod navbar;

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

struct Db {
    users: Vec<User>,
}

impl Db {
    fn new() -> Self {
        Db {
            users: vec![User {
                id: Uuid::new_v4(),
                name: String::from("admin"),
                password: password_auth::generate_hash("admin"),
                level: Level::Admin,
            }],
        }
    }
}

static DB: LazyLock<Db> = LazyLock::new(Db::new);

pub fn shell(options: LeptosOptions) -> impl IntoView {
    view! {
        <!DOCTYPE html>
        <html lang="en">
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
                    <Route path=StaticSegment("/login") view=LoginPage/>
                    <Route path=path!("/dashboard/:id") view=Dashboard/>
                </Routes>
            </main>
        </Router>
    }
}

#[island]
fn Dashboard() -> impl IntoView {
    view! {
        "dashboard"
    }
}

#[component]
fn HomePage() -> impl IntoView {
    view! {
        <div class=r#"bg-[url("/background.jpg")] h-128 w-full bg-cover bg-center"#>
            <Navbar/>
        </div>
    }
}
