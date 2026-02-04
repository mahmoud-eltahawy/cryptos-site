use leptos::prelude::*;

#[component]
pub fn FeaturesSection() -> impl IntoView {
    view! {
         <div id="estates" class="py-20 bg-gradient-to-br from-gray-50 to-blue-50">
             <div class="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
                 <Why/>
                 <div class="grid grid-cols-1 md:grid-cols-3 gap-8">
                     <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105 border border-gray-100">
                         <div class="bg-gradient-to-br from-blue-500 to-cyan-500 w-16 h-16 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300">
                             <HomeIcon/>
                         </div>
                         <h3 class="text-2xl font-bold text-gray-800 mb-4">"عقارات متنوعة"</h3>
                         <p class="text-gray-600 leading-relaxed">"مجموعة واسعة من العقارات السكنية والتجارية التي تناسب جميع الاحتياجات والميزانيات"</p>
                     </div>

                     <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105 border border-gray-100">
                         <div class="bg-gradient-to-br from-purple-500 to-pink-500 w-16 h-16 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300">
                             <ShieldIcon/>
                         </div>
                         <h3 class="text-2xl font-bold text-gray-800 mb-4">"موثوقية عالية"</h3>
                         <p class="text-gray-600 leading-relaxed">"نضمن لك التعامل الآمن والشفاف مع فريق محترف من الخبراء في المجال العقاري"</p>
                     </div>

                     <div class="group bg-white rounded-2xl p-8 shadow-lg hover:shadow-2xl transition-all duration-300 hover:scale-105 border border-gray-100">
                         <div class="bg-gradient-to-br from-green-500 to-emerald-500 w-16 h-16 rounded-2xl flex items-center justify-center mb-6 group-hover:scale-110 transition-transform duration-300">
                             <ThunderIcon/>
                         </div>
                         <h3 class="text-2xl font-bold text-gray-800 mb-4">"خدمة سريعة"</h3>
                         <p class="text-gray-600 leading-relaxed">"نساعدك في إيجاد العقار المناسب بسرعة وكفاءة مع دعم فني على مدار الساعة"</p>
                     </div>
                 </div>
             </div>
         </div>
    }
}

#[component]
fn Why() -> impl IntoView {
    view! {
         <div class="text-center mb-16">
             <h2 class="text-4xl md:text-5xl font-bold bg-gradient-to-r from-blue-600 to-purple-600 bg-clip-text text-transparent mb-4 p-4">
                 "لماذا تختار كريبتوس؟"
             </h2>
             <p class="text-gray-600 text-xl">"نقدم لك أفضل الحلول العقارية"</p>
         </div>
    }
}

#[component]
fn HomeIcon() -> impl IntoView {
    view! {
         <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
             <path
                 stroke-linecap="round"
                 stroke-linejoin="round"
                 stroke-width="2"
                 d="M3 12l2-2m0 0l7-7 7 7M5 10v10a1 1 0 001 1h3m10-11l2 2m-2-2v10a1 1 0 01-1 1h-3m-6 0a1 1 0 001-1v-4a1 1 0 011-1h2a1 1 0 011 1v4a1 1 0 001 1m-6 0h6"></path>
         </svg>
    }
}

#[component]
fn ShieldIcon() -> impl IntoView {
    view! {
         <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
             <path
                 stroke-linecap="round"
                 stroke-linejoin="round"
                 stroke-width="2"
                 d="M9 12l2 2 4-4m5.618-4.016A11.955 11.955 0 0112 2.944a11.955 11.955 0 01-8.618 3.04A12.02 12.02 0 003 9c0 5.591 3.824 10.29 9 11.622 5.176-1.332 9-6.03 9-11.622 0-1.042-.133-2.052-.382-3.016z"></path>
         </svg>
    }
}

#[component]
fn ThunderIcon() -> impl IntoView {
    view! {
         <svg class="w-8 h-8 text-white" fill="none" stroke="currentColor" viewBox="0 0 24 24">
             <path
                 stroke-linecap="round"
                 stroke-linejoin="round"
                 stroke-width="2"
                 d="M13 10V3L4 14h7v7l9-11h-7z"></path>
         </svg>
    }
}
