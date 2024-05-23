use sycamore::prelude::*;
use markdown::*;

#[component]
pub async fn Article<G: Html>(props: ArticleProps) -> View<G> {
    let selected = create_signal(false);
    let toggle_selected = move |_| {
        selected.set(!selected.get());
    };
    let injected = create_signal(String::new());
    create_effect(move || {
        if selected.get() {
            injected.set(props.body.clone())
        } else {
            injected.set(props.hook.clone());
        }
    });

    view! {
        button(
                class="article",
                on:click=toggle_selected,
                data-selected=selected.get()
            ) {
                div(class="header") {
                    div { (props.title) }
                    div { (props.date) }
                }
                div(class="core") {
                    div(dangerously_set_inner_html=to_html_with_options(&injected.get_clone(), &Options::gfm()).unwrap())
                }
            }
    }
}

#[derive(Props)]
pub struct ArticleProps {
    title: String,
    date: String,
    hook: String,
    body: String,
}
