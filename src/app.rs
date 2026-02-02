use std::sync::{LazyLock, Mutex};

use crate::app::{
    dashboard::{
        Dashboard,
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
                    <Route path=path!("/dashboard/addUser/:id") view=AddUser/>
                    <Route path=path!("/dashboard/manageUser/:id") view=ManageUser/>
                    <Route path=path!("/dashboard/:id") view=Dashboard/>
                </Routes>
            </main>
        </Router>
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
