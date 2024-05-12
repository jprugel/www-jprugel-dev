use super::article::Article;
use crate::utility::Feed;
use sycamore::prelude::*;
use sycamore::futures::*;
use gloo_net::http::*;

#[component]
pub async fn Feed<G: Html>() -> View<G> {
    let post_count = create_signal(0);
    on_mount(move || {
        spawn_local_scoped(async move {
            post_count.set(get_post_count().await);
        });
    });
    let articles = create_signal(view! {});
    create_effect(move || {
        let mut articles_vec: Vec<View<G>> = vec![];
        for i in 0..post_count.get() {
            articles_vec.push(view! {Article(id=i)});
        }
        articles.set(View::new_fragment(articles_vec));
    });
    view! {
        div(class="feed") {
            (articles.get_clone())
        }
    }
}

async fn get_post_count() -> usize {
    let posts = Request::get("/blogs")
        .send()
        .await
        .unwrap();

    let feed: Feed = posts.json().await.unwrap();
    feed.blogs.len()
}
