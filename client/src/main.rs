use client::App;

fn main() {

    let root = web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .query_selector("#sycamore")
        .unwrap()
        .unwrap();

    sycamore::hydrate_to(App, &root);
}
