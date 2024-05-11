pub mod components;
pub mod utility;

use components::navbar::*;
use sycamore::prelude::*;
use std::fmt::*;

use components::feed::Feed;

#[component]
pub async fn App<G: Html>() -> View<G> {
    create_memo(move || {
        let dropdown_ctx = create_signal(DropdownContext::new(false));
        provide_context(dropdown_ctx);
        let mouse_is_here = create_signal(MouseIsWithinSettingsMenu(true));
        provide_context(mouse_is_here);
        let theme: Signal<Themes> = create_signal(Themes::Latte);
        provide_context(theme);
        on_mount(move || {
            let local_storage: Option<web_sys::Storage> = web_sys::window()
                .unwrap()
                .local_storage()
                .unwrap();
            if let Some(local_storage) = &local_storage {
                let theme_in_storage: Option<String> = local_storage.get_item("theme")
                    .unwrap();
                if let Some(theme_in_storage) = &theme_in_storage {
                    theme.set(Themes::from_string(theme_in_storage.to_string()));
                } else {
                    let user_pref_dark = web_sys::window()
                        .unwrap()
                        .match_media("(prefers-color-scheme: dark)")
                        .unwrap()
                        .unwrap()
                        .matches();
                    if user_pref_dark {
                        theme.set(Themes::Frappe);
                    } else {
                        theme.set(Themes::Latte);
                    }
                }
            }
        });

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

impl Themes {
    fn from_string(target: String) -> Themes {
        match target.as_str() {
            "latte" => Themes::Latte,
            "frappe" => Themes::Frappe,
            "macchiato" => Themes::Macchiato,
            "mocha" => Themes::Mocha,
            _ => Themes::Latte,
        }
    }
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
