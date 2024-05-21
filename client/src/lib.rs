pub mod components;
pub mod utility;

use components::navbar::*;
use sycamore::prelude::*;

use components::feed::Feed;
use components::login::{
    LoginVisibility,
    LoginMP,
};
use utility::*;

#[component]
pub async fn App<G: Html>() -> View<G> {
    create_memo(move || {
        let settings_visibility = create_signal(DropdownContext::new(false));
        provide_context(settings_visibility);
        let settings_mouse_present = create_signal(MouseIsWithinSettingsMenu(true));
        provide_context(settings_mouse_present);
        let login_mouse_present = create_signal(LoginMP(false));
        provide_context(login_mouse_present);
        let theme: Signal<Themes> = create_signal(Themes::Latte);
        provide_context(theme);
        let login_visibility = create_signal(LoginVisibility(false));
        provide_context(login_visibility);
        let search_query = create_signal(SearchQuery::default());
        provide_context(search_query);

        let handle_click = move |_| {
            if !settings_mouse_present.get().0 {
                settings_visibility.set(DropdownContext::new(false));
            }

            if !login_mouse_present.get().0 {
                login_visibility.set(LoginVisibility(false))
            }
        };

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
