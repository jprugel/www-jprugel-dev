use sycamore::prelude::*;

#[component]
pub fn Article<G: Html>(details: Details) -> View<G> {
    view! {
        button(class="article") {
            Header(title=details.title, date=details.date) {}
            Core(value=details.body) {}
            Footer {}
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
        p(class="core", dangerously_set_inner_html=<String as Clone>::clone(&body.value))
    }
}

#[component]
pub fn Footer<G: Html>() -> View<G> {
    view! {}
}

#[derive(Props)]
pub struct Details {
    pub id: i32,
    pub title: String,
    pub date: String,
    pub body: String,
}

#[derive(Props)]
pub struct HeaderProps {
    pub title: String,
    pub date: String,
}

#[derive(Props)]
pub struct BodyProps {
    pub value: String,
}
