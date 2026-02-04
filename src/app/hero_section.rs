use leptos::prelude::*;
#[component]
pub fn HeroSection() -> impl IntoView {
    view! {
        <div class=r#"relative bg-gradient-to-br from-blue-600 via-purple-600 to-pink-600 min-h-screen flex items-center justify-center overflow-hidden"#>
            <div class="absolute inset-0 bg-[url('/background.jpg')] bg-cover bg-center opacity-20"></div>
            <div class="absolute inset-0 bg-gradient-to-br from-blue-900/50 to-purple-900/50"></div>

            // Animated circles
            <div class="absolute top-20 left-20 w-72 h-72 bg-blue-400/30 rounded-full blur-3xl animate-pulse"></div>
            <div class="absolute bottom-20 right-20 w-96 h-96 bg-purple-400/30 rounded-full blur-3xl animate-pulse delay-700"></div>

            <div class="relative z-10 max-w-6xl mx-auto px-4 sm:px-6 lg:px-8 text-center">
                <div class="animate-fade-in">
                    <PageTitle/>
                    <Description/>
                    <ButtonsRow/>
                </div>
                <Stats/>
            </div>
        </div>
    }
}

#[component]
fn LoginFancyButton() -> impl IntoView {
    view! {
        <a
            href="/login"
            class="px-8 py-4 bg-transparent border-2 border-white text-white font-bold text-lg rounded-full hover:bg-white hover:text-blue-600 shadow-xl hover:shadow-2xl hover:scale-110 transition-all duration-300"
        >
            "تسجيل الدخول"
        </a>
    }
}

#[component]
fn Description() -> impl IntoView {
    view! {
        <p class="text-xl md:text-2xl text-blue-100 mb-12 max-w-3xl mx-auto leading-relaxed">
            "نساعدك في العثور على العقار المثالي واستثمار أموالك بذكاء"
        </p>
    }
}

#[component]
fn PageTitle() -> impl IntoView {
    view! {
        <h1 class="text-5xl md:text-7xl font-bold text-white mb-6 leading-tight">
            "كريبتوس للتسويق"
            <br/>
            <span class="bg-gradient-to-r from-yellow-300 to-pink-300 bg-clip-text text-transparent">
                "والاستثمار العقاري"
            </span>
        </h1>
    }
}

#[component]
fn ButtonsRow() -> impl IntoView {
    view! {
        <div class="flex flex-wrap gap-6 justify-center">
            <ExploreButton/>
            <LoginFancyButton/>
        </div>
    }
}

#[component]
fn ExploreButton() -> impl IntoView {
    view! {
        <a
            href="#estates"
            class="group px-8 py-4 bg-white text-blue-600 font-bold text-lg rounded-full shadow-2xl hover:shadow-white/20 hover:scale-110 transition-all duration-300 flex items-center gap-3"
        >
            "استكشف العقارات"
            <LeftArrowIcon/>
        </a>
    }
}

#[component]
fn LeftArrowIcon() -> impl IntoView {
    view! {
        <svg class="w-6 h-6 group-hover:translate-x-2 transition-transform duration-300" fill="none" stroke="currentColor" viewBox="0 0 24 24">
            <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M13 7l5 5m0 0l-5 5m5-5H6"></path>
        </svg>
    }
}

#[component]
fn Stats() -> impl IntoView {
    view! {
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
    }
}
