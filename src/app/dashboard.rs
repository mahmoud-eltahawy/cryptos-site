use leptos::prelude::*;

#[island]
pub fn Dashboard() -> impl IntoView {
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3">
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
            <Card/>
        </div>
    }
}

#[component]
fn Card() -> impl IntoView {
    view! {
        <button class="hover:text-lime-600 text-xl md:text-2xl lg:text-3xl hover:text-2xl hover:md:text-3xl hover:lg:text-4xl border-2 hover:border-lime-700 hover:border-5 hover:rounded-3xl h-32 justify-items-stretch mx-2 my-5 rounded-lg">"button"</button>
    }
}
