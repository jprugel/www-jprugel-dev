use sycamore::prelude::*;

#[component]
pub fn Article<G: Html>() -> View<G> {
    view! {
        p(class="pain") { "%db_info.body" }
    }
}

#[derive(Props)]
pub struct Details {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub description: String,
}
