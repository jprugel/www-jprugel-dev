use sycamore::prelude::*;
use crate::utility::SearchQuery;
use wasm_bindgen::JsValue;

const SEARCH: &str = "m19.6 21l-6.3-6.3q-.75.6-1.725.95T9.5 16q-2.725 0-4.612-1.888T3 9.5t1.888-4.612T9.5 3t4.613 1.888T16 9.5q0 1.1-.35 2.075T14.7 13.3l6.3 6.3zM9.5 14q1.875 0 3.188-1.312T14 9.5t-1.312-3.187T9.5 5T6.313 6.313T5 9.5t1.313 3.188T9.5 14";

#[component]
pub fn Search<G: Html>() -> View<G> {
    let search_query = use_context::<Signal<SearchQuery>>();
    let handle = move |e: web_sys::Event| {
        let target = e.target().expect("Failed to capture target @ search.rs");
        let input = web_sys::HtmlInputElement::from(JsValue::from(target));
        search_query.set(SearchQuery::new(&input.value()));
    };
    view! {
        div(
            class="search-container",
        ) {
            svg(
                class="search-svg",
                xmlns="http://www.w3.org/2000/svg",
                width="1em", 
                height="1em", 
                viewBox="0 0 24 24",
                preserveAspectRatio="none",
            ) {
                path(
                    fill="currentColor", 
                    d=SEARCH,
                )
            }
            label {  
                input(
                    class="search",
                    type="search",
                    paceholder="search",
                    on:input=handle,
                )
            }
        }
    }
}
