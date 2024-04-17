use sycamore::prelude::*;

#[component]
pub fn App<G: Html>() -> View<G> {
    create_memo(move || {
        let theme = create_signal(Theme("latte"));
        provide_context(theme.get());

        view! {
            div(data-theme=theme.get().0) {
                Navbar {}
            }
        }
    }).get_clone()
}

#[component]
pub fn Navbar<G: Html>() -> View<G> {
    view! {
        div(class="navbar") {
            Title {}
            ThemeButton {}
            Favicons {}
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
    let change_theme = move |_| match theme.get() {
        Theme("latte") => theme.set(Theme("frappe")),
        Theme("frappe") => theme.set(Theme("macchiato")),
        Theme("macchiato") => theme.set(Theme("mocha")),
        Theme("mocha") => theme.set(Theme("frappe")),
        _ => theme.set(Theme("null")),
    };

    view! {
        button(on:click=change_theme) {"theme"}
    }
}

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub struct Theme(&'static str);


