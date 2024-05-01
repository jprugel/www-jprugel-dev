use sycamore::prelude::*;

#[component]
pub fn ArticleButton<G: Html>(details: Details) -> View<G> {
    view! {
        button(class="article") {
            Header(title=details.title, date=details.date) {}
            Core(summary=details.summary, body=details.body) {}
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
