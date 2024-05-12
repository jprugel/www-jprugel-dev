use sycamore::prelude::*;
use sycamore::futures::*;
use gloo_net::http::*;
use crate::utility::*;

#[component]
pub async fn Article<G: Html>(props: ArticleProps) -> View<G> {
    let post = create_signal(Post::default());
    on_mount(move || {
        spawn_local_scoped(async move {
            let p = get_post(props.id).await;
            post.set(
                Post::builder()
                    .set_id(p.get_id())
                    .set_title(&p.get_title())
                    .set_date(&p.get_date())
                    .set_body(&p.get_body())
                    .build()
            );
        });
    });
    let selected = create_signal(ArticleSelected(false));
    let toggle_selected = move |_| {
    };
    let summary = create_signal(String::new());
    create_effect(move || {
        let body_clone = post.get_clone().get_body();
        if body_clone.len() >= 1 {
            let summary_as_vec: Vec<String> = body_clone
                .split_inclusive("  ")
                .map(|x| x.to_string())
                .collect();
            if summary_as_vec.len() >= 2 {
                summary.set(summary_as_vec[1].clone());
            } else {
                summary.set(body_clone);
            }
        }
    });
    view! {
         button(
            class="article", 
            on:mouseout=toggle_selected,
            data-selected=selected.get().0
        ) {
            div(class="header") {
                p { (post.get_clone().get_title()) }
                p { (post.get_clone().get_date()) }
            }
            div(class="core") {
                p(dangerously_set_inner_html=markdown::to_html(&summary.get_clone()))
            }
        }   
    }
}

#[derive(Props)]
pub struct ArticleProps {
    pub id: usize,
}

async fn get_post(id: usize) -> Post {
    let blogs = Request::get("/blogs")
        .send()
        .await
        .unwrap();

    let feed: Feed = blogs.json().await.expect("failed to parse to feed");
    feed.blogs.get(id).expect("0th post does not exist").clone()
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub struct ArticleSelected(bool);
