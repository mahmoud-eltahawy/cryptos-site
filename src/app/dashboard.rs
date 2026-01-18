use leptos::prelude::*;
use leptos_router::hooks::use_params_map;
use uuid::Uuid;

pub mod add_user;
pub mod remove_user;

#[component]
pub fn Dashboard() -> impl IntoView {
    let params = use_params_map();
    let user_id = move || params.with(|p| p.get("id"));
    view! {
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 justify-items-stretch">
            <Card name="اضافة مستخدم" href={format!("/dashboard/addUser/{}",user_id().unwrap_or("".to_string()))}/>
            <Card name="حذف مستخدم" href={format!("/dashboard/removeUser/{}",user_id().unwrap_or("".to_string()))}/>
        </div>
    }
}

#[component]
fn Card(href: String, name: &'static str) -> impl IntoView {
    view! {
        <a
            class="flex items-center text-center hover:text-lime-600 text-xl md:text-2xl lg:text-3xl hover:text-2xl hover:md:text-3xl hover:lg:text-4xl border-2 hover:border-lime-700 hover:border-5 hover:rounded-3xl h-32 mx-2 my-5 rounded-lg"
            href={href}
        >
           <p class="text-center w-full">{name}</p>
        </a>
    }
}

#[server]
async fn get_users_names() -> Result<Vec<(Uuid, String)>, ServerFnError> {
    let xs = crate::app::DB
        .users
        .lock()
        .unwrap()
        .iter()
        .map(|x| (x.id, x.name.clone()))
        .collect::<Vec<_>>();
    Ok(xs)
}
