use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>() -> View<G> {
    view! {
        div(class="navbar") {
            Title {}
            ThemeButton {}
            Favicons {}
            Settings {} 
        }
    }
}

#[component]
pub fn Title<G: Html>() -> View<G> {
    view! {
        div(class="title") {
            "JPRUGEL"
        }
    }
}

#[component]
pub fn Favicons<G: Html>() -> View<G> {
    view! {
        div(class="favicons") {
            a(href="https://www.github.com/jprugel") {
                img(src="https://www.github.com/favicon.ico", alt="", class="favicon") {}
            }
        }
    }
}

#[component]
pub fn ThemeButton<G: Html>() -> View<G> {
    let theme = use_context::<Signal<Theme>>();
    let change_theme = move |_| match theme.get().value {
        "latte" => theme.set(Theme::new("frappe")),
        "frappe" => theme.set(Theme::new("macchiato")),
        "macchiato" => theme.set(Theme::new("mocha")),
        "mocha" => theme.set(Theme::new("latte")),
        _ => theme.set(Theme::new("null")),
    };

    view! {
        button(class="theme", on:click=change_theme) {"theme"}
    }
}

#[component]
pub fn Settings<G: Html>() -> View<G> {
    static SETTINGS_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="1em" height="1em" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 12a1 1 0 1 0 2 0a1 1 0 1 0-2 0m7 0a1 1 0 1 0 2 0a1 1 0 1 0-2 0m7 0a1 1 0 1 0 2 0a1 1 0 1 0-2 0"/></svg>"#;

    view! {
        div(
            class="settings", 
            dangerously_set_inner_html=SETTINGS_SVG,
        )
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Theme {
    pub value: &'static str,
}

impl Theme {
    pub fn new(value: &'static str) -> Self {
        Self {
            value,
        }
    }
}
