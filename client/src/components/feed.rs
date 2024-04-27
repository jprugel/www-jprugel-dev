use sycamore::prelude::*;

#[component]
pub fn Feed<G: Html>() -> View<G> {
    view! {
        div(class="feed") { "%db_info.body" }
    }
}
