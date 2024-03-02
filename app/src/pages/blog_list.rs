use bevy_ecs::prelude::{Query, Without};
use bevy_ecs::query::With;
use cinnog::{run_system, FileName};
use leptos::*;
use leptos_router::use_params_map;

use crate::data::post::*;

struct PostSummary {
  title: String,
  description: String,
  href: String,
}

#[component]
fn PostCard(post: PostSummary) -> impl IntoView {
  let href = post.href;
  view! {
    <div class="flex flex-col bg-white border shadow-lg h-full rounded-xl">
    // <img class="w-full h-auto rounded-t-xl" src="https://images.unsplash.com/photo-1680868543815-b8666dba60f7?ixlib=rb-4.0.3&ixid=MnwxMjA3fDB8MHxwaG90by1wYWdlfHx8fGVufDB8fHx8&auto=format&fit=crop&w=2532&q=80" alt="Image Description">
      <a class="block p-4 md:p-5" href=href>
        <h3 class="text-lg font-bold text-black">
          {post.title}
        </h3>
        <p class="mt-1 text-black">
          {post.description}
            //"Some quick example text to build on the card title and make up the bulk of the card's content."
        </p>
        /* <a class="mt-2 py-2 px-3 inline-flex justify-center shadow-lg items-center gap-x-2 text-sm font-semibold rounded-lg border border-transparent bg-primary text-white hover:bg-primary[1.5] disabled:opacity-50 disabled:pointer-events-none dark:focus:outline-none" href=href>
          "Post"
        </a>*/
      </a>
    </div>
  }
}

#[component]
pub fn BlogPostsPage() -> impl IntoView {
  let posts = run_system(get_posts);

  let posts_view = posts.into_iter().map(|post| view! {
    <PostCard post />
  }).collect_view();

  view! {
    <div>
      <h1 class="text-2xl">"Projects"</h1>
      <div class="grow grid sm:grid-cols-2 lg:grid-cols-3 gap-8">
        {posts_view}
      </div>
    </div>
  }
}

fn get_posts(
    posts: Query<(&PostTitle, &PostDescription, &Slug), (With<Post>, Without<Draft>)>,
) -> Vec<PostSummary> {
    posts
      .iter()
      .map(|(title, desc, slug)| PostSummary {
        title: title.0.clone(),
        description: desc.0.clone(),
        href: format!("/blog/{}", slug.0)
      })
      .collect()
}
