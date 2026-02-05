use leptos::prelude::*;

#[component]
pub fn Navbar() -> impl IntoView {
    view! {
        <nav class="bg-white/95 backdrop-blur-md shadow-lg border-b border-gray-200 sticky top-0 z-50">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="flex justify-between items-center h-20">
                    <Logo/>
                    <div class="flex items-center gap-4">
                        <a
                            href="/estates"
                            class="text-gray-700 hover:text-blue-600 font-semibold px-4 py-2 rounded-lg transition-all duration-300 hover:bg-blue-50"
                        >
                            "عرض العقارات"
                        </a>
                        <Login/>
                    </div>
                </div>
            </div>
        </nav>
    }
}

#[component]
fn Logo() -> impl IntoView {
    view! {
        <a
            class="flex items-center gap-3 text-2xl font-bold text-gray-800 hover:text-blue-600 transition-colors duration-300"
            href="/"
        >
            <div class="bg-gradient-to-br from-blue-600 to-purple-600 p-2 rounded-xl shadow-lg">
                <img width="40" height="40" src="black-logo.png" alt="logo" class="brightness-0 invert"/>
            </div>
            <span class="bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent">
                "Cryptos"
            </span>
        </a>
    }
}

#[component]
fn Login() -> impl IntoView {
    view! {
        <a
            class="px-6 py-2.5 bg-gradient-to-r from-blue-600 to-purple-600 text-white font-semibold rounded-lg shadow-md hover:shadow-xl hover:scale-105 transition-all duration-300"
            href="/login"
        >
            "تسجيل دخول"
        </a>
    }
}
