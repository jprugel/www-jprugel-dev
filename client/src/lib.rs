pub mod components;

use sycamore::prelude::*;
use components::navbar::*;
use components::feed::*;

#[component]
pub fn App<G: Html>() -> View<G> {
    create_memo(move || {
        let theme = create_signal(Theme::new("latte"));
        provide_context(theme);

        view! {
            div(class="app", data-theme=theme.get().value) {
                Navbar {}
                Feed {}
            }
        }
    })
    .get_clone()
}

