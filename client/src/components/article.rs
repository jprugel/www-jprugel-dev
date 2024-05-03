use sycamore::prelude::*;
use wasm_bindgen::JsValue;

#[component]
pub fn Article<G: Html>(details: Details) -> View<G> {
    let selected = create_signal(ArticleSelected(false));
    let toggle_selected = move |_| {
        web_sys::console::log_1(&JsValue::from_str("work?"));
    };

    view! {
        button(
            class="article", 
            on:mouseout=toggle_selected,
            data-selected=selected.get().0
        ) {
            Header(title=details.title, date=details.date)
            Core(summary=details.summary, body=details.body)
        }
    }
}

#[component]
pub fn Header<G: Html>(props: HeaderProps) -> View<G> {
    view! {
        div(class="header") {
            p { (props.title) }
            p { (props.date) }
        }
    }
}

#[component]
pub fn Title<G: Html>() -> View<G> {
    view! {}
}

#[component]
pub fn Date<G: Html>() -> View<G> {
    view! {}
}

#[component]
pub fn Core<G: Html>(body: BodyProps) -> View<G> {
    view! {
        p(class="core", dangerously_set_inner_html=<String as Clone>::clone(&body.summary))
    }
}

#[derive(Props)]
pub struct Details {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub summary: String,
    pub body: String,
}

#[derive(Props)]
pub struct HeaderProps {
    pub title: String,
    pub date: String,
}

#[derive(Props)]
pub struct BodyProps {
    pub summary: String,
    pub body: String,
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ArticleSelected(bool);
