use std::sync::LazyLock;

use leptos::{ev::Event, prelude::*};
use leptos_meta::{MetaTags, Stylesheet, Title, provide_meta_context};
use leptos_router::{
    StaticSegment,
    components::{Route, Router, Routes},
    path,
};
use navbar::Navbar;
use uuid::Uuid;

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
            <body>
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

#[server]
async fn login(username: String, password: String) -> Result<(), ServerFnError> {
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
const MINIMUM_USERNAME_LEN: usize = 3;
const MINIMUM_PASSWORD_LEN: usize = 3;

#[island]
fn LoginPage() -> impl IntoView {
    let username = RwSignal::new(String::new());
    let password = RwSignal::new(String::new());
    let on_username = move |ev: Event| {
        let value = event_target_value(&ev);
        *username.write() = value;
    };
    let on_password = move |ev: Event| {
        let value = event_target_value(&ev);
        *password.write() = value;
    };
    let valid_username = move || username.read().len() > MINIMUM_USERNAME_LEN;
    let valid_password = move || password.read().len() > MINIMUM_PASSWORD_LEN;
    let valid_login = move || valid_username() && valid_password();

    let login_ac = ServerAction::<Login>::new();

    view! {
        <div class="border-5 rounded-lg m-20 p-5 text-center">
            <ActionForm action={login_ac}>
                <div class="grid grid-cols-1">
                    <label class="text-2xl">"اسم المستخدم"</label>
                    <input
                        style={move || if valid_username() {""} else {"color:red;"}}
                        on:input={on_username}
                        class="text-center my-5 mx-20 p-5 border-2 rounded-lg"
                        type="text"
                        name="username"
                        value={move|| username.get()}
                    />
                </div>
                <div class="grid grid-cols-1">
                    <label class="text-2xl">"كلمة السر"</label>
                    <input
                        style={move || if valid_password() {""} else {"color:red;"}}
                        on:input={on_password}
                        class="text-center my-5 mx-20 p-5 border-2 rounded-lg"
                        type="password"
                        name="password"
                        value={move|| password.get()}
                    />
                </div>
                <input
                    prop:disabled={move || !valid_login()}
                    class="text-center my-5 mx-20 p-5 border-2 rounded-lg hover:text-3xl hover:bg-green-600 hover:cursor-pointer text-xl disabled:text-sm disabled:text-slate-300"
                    type="submit"
                    value="تأكيد"
                />
            </ActionForm>
        </div>
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
