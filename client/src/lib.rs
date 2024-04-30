pub mod components;

use components::feed::*;
use components::navbar::*;
use sycamore::prelude::*;

#[component]
pub fn App<G: Html>() -> View<G> {
    create_memo(move || {
        let theme = create_signal(Theme::new("latte"));
        provide_context(theme);
        let dropdown_ctx = create_signal(DropdownContext::new(false));
        provide_context(dropdown_ctx);
        let mouse_is_here = create_signal(MouseIsWithinSettingsMenu(true));
        provide_context(mouse_is_here);

        let handle_click = move |_| {
            if !mouse_is_here.get().0 {
                dropdown_ctx.set(DropdownContext::new(false));
            }
        };

        view! {
            div(
                class="app",
                data-theme=theme.get().value,
                on:click=handle_click,
            ) {
                Navbar {}
                Feed {}
            }
        }
    })
    .get_clone()
}
