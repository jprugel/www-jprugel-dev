use super::article::Article;
use crate::utility::Feed;
use sycamore::prelude::*;
use sycamore::futures::*;
use gloo_net::http::*;
use crate::utility::SearchQuery;

#[component]
pub fn Feed<G: Html>() -> View<G> {
    let feed = create_signal(Feed::default());
    let search_query = use_context::<Signal<SearchQuery>>();
    on_mount(move || {
        spawn_local_scoped(async move {
            feed.set(get_feed_from_server().await);
        });
    });

    let feed_view = create_signal(view! {});
    create_effect(move || {
        let articles: Vec<View<G>> = feed.get_clone()
            .articles
            .into_iter()
            .filter(|x| {
                x.body.to_lowercase().contains(search_query.get_clone().0.to_lowercase().as_str())
            })
            .map(|x| view! {Article(
                title=x.title,
                date=x.date,
                hook=x.hook,
                body=x.body,
            )})
            .collect();

        feed_view.set(View::new_fragment(articles));
    });

    view! {
        div(class="feed") {
            (feed_view.get_clone())
        }
    }
}

async fn get_feed_from_server() -> Feed {
    let feed = Request::get("/feed")
        .send()
        .await
        .expect("Failed to GET feed");

    feed.json()
        .await
        .expect("Failed to parse feed")
}
