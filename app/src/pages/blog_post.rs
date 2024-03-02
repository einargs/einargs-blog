use bevy_ecs::prelude::{In, Query};
use cinnog::{run_system_with_input, FileName};
use cinnog::loaders::markdown::Html;
use leptos::*;
use leptos_router::{use_params, Params};

use crate::data::post::*;

#[derive(PartialEq, Debug, Clone, Params)]
struct BlogPostParams {
  slug: String
}

struct PostInfo {
  title: String,
  description: String,
  content: Html,
}

#[component]
pub fn BlogPostPage() -> impl IntoView {
  let params = use_params::<BlogPostParams>()
    .get()
    .unwrap();
  let current_slug = params.slug;

  let post = run_system_with_input(get_post, current_slug);
  let title: String = post.title;
  let content: String = post.content.0;

  view! {
    <div>
      <h1 class="text-2xl">{title}</h1>
      <div class="post-markdown" inner_html=content></div>
    </div>
  }
}

fn get_post(
  In(this_slug): In<String>,
  posts: Query<(&Html, &Slug, &PostTitle, &PostDescription)>,
) -> PostInfo {
  let (html, slug, title, desc) = posts
    .iter()
    .find(|(_, slug, _, _)| slug.0 == this_slug)
    .expect("posts");

  PostInfo {
    title: title.0.clone(),
    description: desc.0.clone(),
    content: html.clone(),
  }
}
