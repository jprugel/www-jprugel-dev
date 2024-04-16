use sycamore::prelude::*;

#[component]
pub fn App<G: Html>() -> View<G> {
    let mut value = create_signal(0i32);
    let increment = move |_| value += 1;
    view! {
        button(on:click=increment) {"Clicks: " (value.get())}
    }
}
