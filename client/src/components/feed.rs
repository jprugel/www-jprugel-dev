use super::article::Article;
use sycamore::prelude::*;
use sycamore::suspense::*;

#[component]
pub fn Feed<G: Html>() -> View<G> {
    view! {
        div(class="feed") {
            Suspense(fallback = view! {p{"loading"}}) {
                Article
            }
        }
    }
}
