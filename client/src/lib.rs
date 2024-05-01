pub mod components;

use components::feed::*;
use components::navbar::*;
use sycamore::prelude::*;
use std::fmt::*;

#[component]
pub fn App<G: Html>() -> View<G> {
    create_memo(move || {
        let theme = create_signal(Themes::Latte);
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
                data-theme=theme.get().to_string(),
                on:click=handle_click,
            ) {
                Navbar {}
                Feed {}
            }
        }
    })
    .get_clone()
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum Themes {
    Latte,
    Frappe,
    Macchiato,
    Mocha,
}

impl Display for Themes {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            Themes::Latte => write!(f, "latte"),
            Themes::Frappe => write!(f, "frappe"),
            Themes::Macchiato => write!(f, "macchiato"),
            Themes::Mocha => write!(f, "mocha"),
        }
    }
}
