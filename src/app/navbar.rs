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
pub fn Footer() -> impl IntoView {
    view! {
        <footer class="bg-gradient-to-r from-gray-900 to-gray-800 text-white py-12">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="grid grid-cols-1 md:grid-cols-3 gap-10 items-start text-center md:text-right" dir="rtl">

                    // Column 1: Branding
                    <div class="flex flex-col items-center md:items-start gap-3">
                        <div class="flex items-center gap-3">
                            <Logo/>
                        </div>
                        <p class="text-gray-400 mt-2 text-sm leading-relaxed">
                            "كريبتوس للتسويق والاستثمار والتطوير العقاري"
                            <br/>
                            "شريكك العقاري الموثوق لبناء المستقبل."
                        </p>
                    </div>

                    // Column 2: Contact Info
                    <div class="flex flex-col gap-4">
                        <h3 class="text-lg font-bold border-b border-gray-700 pb-2 mb-2">"اتصل بنا"</h3>
                        <a href="tel:+966500000000" class="text-gray-400 hover:text-white transition-colors">
                            "📞 +966 50 000 0000"
                        </a>
                        <p class="text-gray-400">
                            "📍 القاهرة، جمهورية مصر العربية"
                        </p>
                    </div>

                    // Column 3: Quick Links or Social
                    <div class="flex flex-col gap-4">
                        <h3 class="text-lg font-bold border-b border-gray-700 pb-2 mb-2">"روابط سريعة"</h3>
                        <div class="flex flex-col gap-2">
                            <a href="/estates" class="text-gray-400 hover:text-white">"العقارات"</a>
                            <a href="/about" class="text-gray-400 hover:text-white">"عن الشركة"</a>
                        </div>
                    </div>

                </div>

                <div class="mt-12 pt-8 border-t border-gray-700 text-center text-gray-500 text-xs">
                    <p>"© " {2026} " كريبتوس. جميع الحقوق محفوظة."</p>
                </div>
            </div>
        </footer>
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
                <img width="40" height="40" src="/black-logo.png" alt="logo" class="brightness-0 invert"/>
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

#[component]
pub fn About() -> impl IntoView {
    view! {
        <section class="py-20 bg-white min-h-screen">
            <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                <div class="w-full md:w-1/2 space-y-6 text-right">
                    <h2 class="text-3xl md:text-4xl font-extrabold text-gray-900 leading-tight">
                        "نبذة عن "
                        <span class="text-blue-600">"كريبتوس"</span>
                    </h2>

                    <p class="text-lg text-gray-600 leading-relaxed">
                        "في كريبتوس، نحن لا نبيع العقارات فحسب، بل نبني جسوراً من الثقة. تخصصنا في التسويق والاستثمار والتطوير العقاري جعلنا الخيار الأول للمستثمرين الباحثين عن التميز والفرص الحقيقية في السوق المصري."
                    </p>

                    <div class="grid grid-cols-1 sm:grid-cols-2 gap-4 pt-4">
                        <div class="flex items-start gap-3">
                            <span class="text-blue-600">"✔️"</span>
                            <div>
                                <h4 class="font-bold text-gray-800">"رؤية واضحة"</h4>
                                <p class="text-sm text-gray-500">"نحلل السوق بدقة لنضمن لك أعلى العوائد."</p>
                            </div>
                        </div>
                        <div class="flex items-start gap-3">
                            <span class="text-blue-600">"✔️"</span>
                            <div>
                                <h4 class="font-bold text-gray-800">"التزام كامل"</h4>
                                <p class="text-sm text-gray-500">"الشفافية هي أساس تعاملنا مع كل عميل."</p>
                            </div>
                        </div>
                    </div>

                    <div class="pt-6">
                        <a href="#contact" class="inline-block bg-gray-900 text-white px-8 py-3 rounded-lg font-medium hover:bg-gray-800 transition-all">
                            "اكتشف المزيد"
                        </a>
                    </div>
                </div>
            </div>
        </section>
    }
}
