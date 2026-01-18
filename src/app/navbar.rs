use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <div class="flex gap-5">
            <Logo/>
            <button class="border-2 rounded-lg p-1 hover:bg-green-700">عرض العقارات</button>
            <Login/>
        </div>
    }
}

#[component]
fn Logo() -> impl IntoView {
    view! {
        <a
            class="flex text-xl size-fit"
            href="/"
        >
            <img width="33" src="black-logo.png" alt="logo"/>
            "Cryptos"
        </a>
    }
}

#[component]
fn Login() -> impl IntoView {
    view! {
        <a
            class="texl-xl border-2 rounded-lg p-1 hover:bg-green-700"
            href="/login"
        >تسجيل دخول</a>
    }
}
