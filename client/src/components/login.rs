use sycamore::prelude::*;

#[component]
pub fn Login<G: Html>() -> View<G> {
    let is_visible = use_context::<Signal<LoginVisibility>>();
    let mouse_present = use_context::<Signal<LoginMP>>();

    view! {
        div(
            class="login",
            data-visibility=is_visible.get().0,
            on:mouseenter=move |_| mouse_present.set(LoginMP(true)),
            on:mouseout=move |_| mouse_present.set(LoginMP(false)),
        ) {
            h3(class="login-title") { "Login" }
            label(class="login-label") {
                "Username: "
                input(class="login-items", type="username")
            }
            label(class="login-label") {
                "Password: "
                input(class="login-items", type="password")
            }
            button(class="sub", type="submit") { "submit" }
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct LoginVisibility(pub bool);

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct LoginMP(pub bool);
