use sycamore::prelude::*;

#[component]
pub fn Navbar<G: Html>() -> View<G> {
    view! {
        div(class="navbar") {
            Title {}
            Favicons {}
            DropdownMenu {}
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
    static GITHUB: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="2.4em" height="2.4em" viewBox="0 0 24 24"><path fill="currentColor" d="M12 2A10 10 0 0 0 2 12c0 4.42 2.87 8.17 6.84 9.5c.5.08.66-.23.66-.5v-1.69c-2.77.6-3.36-1.34-3.36-1.34c-.46-1.16-1.11-1.47-1.11-1.47c-.91-.62.07-.6.07-.6c1 .07 1.53 1.03 1.53 1.03c.87 1.52 2.34 1.07 2.91.83c.09-.65.35-1.09.63-1.34c-2.22-.25-4.55-1.11-4.55-4.92c0-1.11.38-2 1.03-2.71c-.1-.25-.45-1.29.1-2.64c0 0 .84-.27 2.75 1.02c.79-.22 1.65-.33 2.5-.33s1.71.11 2.5.33c1.91-1.29 2.75-1.02 2.75-1.02c.55 1.35.2 2.39.1 2.64c.65.71 1.03 1.6 1.03 2.71c0 3.82-2.34 4.66-4.57 4.91c.36.31.69.92.69 1.85V21c0 .27.16.59.67.5C19.14 20.16 22 16.42 22 12A10 10 0 0 0 12 2"/></svg>"#;
    static LINKEDIN: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="2.6em" height="2.6em" viewBox="0 0 24 24"><path fill="currentColor" d="M19 3a2 2 0 0 1 2 2v14a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2zm-.5 15.5v-5.3a3.26 3.26 0 0 0-3.26-3.26c-.85 0-1.84.52-2.32 1.3v-1.11h-2.79v8.37h2.79v-4.93c0-.77.62-1.4 1.39-1.4a1.4 1.4 0 0 1 1.4 1.4v4.93zM6.88 8.56a1.68 1.68 0 0 0 1.68-1.68c0-.93-.75-1.69-1.68-1.69a1.69 1.69 0 0 0-1.69 1.69c0 .93.76 1.68 1.69 1.68m1.39 9.94v-8.37H5.5v8.37z"/></svg>"#;
    static KOFI: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="2.70em" height="2.7em" viewBox="0 0 32 32"><path fill="currentColor" d="M31.844 11.932c-1.032-5.448-6.48-6.125-6.48-6.125H.964C.156 5.807.057 6.87.057 6.87S-.052 16.637.03 22.637c.22 3.228 3.448 3.561 3.448 3.561s11.021-.031 15.953-.067c3.251-.568 3.579-3.423 3.541-4.98c5.808.323 9.896-3.776 8.871-9.219zm-14.751 4.683c-1.661 1.932-5.348 5.297-5.348 5.297s-.161.161-.417.031c-.099-.073-.14-.12-.14-.12c-.595-.588-4.491-4.063-5.381-5.271c-.943-1.287-1.385-3.599-.119-4.948c1.265-1.344 4.005-1.448 5.817.541c0 0 2.083-2.375 4.625-1.281c2.536 1.095 2.443 4.016.963 5.751m8.23.636c-1.24.156-2.244.036-2.244.036V9.714h2.359s2.631.735 2.631 3.516c0 2.552-1.313 3.557-2.745 4.021z"/></svg>"#;

    view! {
        div(class="favicons") {
            a(class="favicon", target="_blank", href="https://www.github.com/jprugel") {
                div(
                    dangerously_set_inner_html=GITHUB,
                )
            }
            a(class="favicon", target="_blank", href="https://www.linkedin.com/in/jonathon-prugel/") {
                div(
                    dangerously_set_inner_html=LINKEDIN,
                )
            }
            a(class="favicon ko-fi", target="_blank", href="https://ko-fi.com/jprugel") {
                div(
                    dangerously_set_inner_html=KOFI,
                )
            }
        }
    }
}

#[component]
pub fn Settings<G: Html>() -> View<G> {
    static SETTINGS_SVG: &str = r#"<svg xmlns="http://www.w3.org/2000/svg" width="2.5em" height="2.5em" viewBox="0 0 24 24"><path fill="none" stroke="currentColor" stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M4 12a1 1 0 1 0 2 0a1 1 0 1 0-2 0m7 0a1 1 0 1 0 2 0a1 1 0 1 0-2 0m7 0a1 1 0 1 0 2 0a1 1 0 1 0-2 0"/></svg>"#;
    let dropdown_ctx = use_context::<Signal<DropdownContext>>();
    let mouse_is_here = use_context::<Signal<MouseIsWithinSettingsMenu>>();
    let open_dropdown = move |_| {
        mouse_is_here.set(MouseIsWithinSettingsMenu(true));
        dropdown_ctx.set(DropdownContext::new(!dropdown_ctx.get().is_opened));
    };
    view! {
        a(
            class="visible_marker",
            on:click=open_dropdown,
        ) {
            div(
                dangerously_set_inner_html=SETTINGS_SVG
            )
        }
    }
}

#[component]
pub fn DropdownMenu<G: Html>() -> View<G> {
    let theme = use_context::<Signal<Theme>>();
    let change_theme = move |_| match theme.get().value {
        "latte" => theme.set(Theme::new("frappe")),
        "frappe" => theme.set(Theme::new("macchiato")),
        "macchiato" => theme.set(Theme::new("mocha")),
        "mocha" => theme.set(Theme::new("latte")),
        _ => theme.set(Theme::new("null")),
    };
    let dropdown_ctx = use_context::<Signal<DropdownContext>>();
    let mouse_is_here = use_context::<Signal<MouseIsWithinSettingsMenu>>();
    view! {
        div(
            class="dropdown_menu_container",
        ) {
            Settings {}
            div(
                class="dropdown_menu",
                data-visibility=dropdown_ctx.get().is_opened,
                on:mouseout=move |_| {
                    mouse_is_here.set(MouseIsWithinSettingsMenu(false));
                },
                on:mouseenter=move |_| {
                    mouse_is_here.set(MouseIsWithinSettingsMenu(true))
                },
            ) {
                a(
                    on:click=change_theme,
                    class="dropdown"
                ) {
                    "Theme"
                }
                a(
                    href="#"
                ) {
                    "Login"
                }
            }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct Theme {
    pub value: &'static str,
}

impl Theme {
    pub fn new(value: &'static str) -> Self {
        Self { value }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct DropdownContext {
    is_opened: bool,
}

impl DropdownContext {
    pub fn new(is_opened: bool) -> Self {
        Self { is_opened }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct MouseIsWithinSettingsMenu(pub bool);
