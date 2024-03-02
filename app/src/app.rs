use bevy_ecs::prelude::{Query, With};
use bevy_ecs::query::Without;
use cinnog::run_system;
use leptos::*;
use leptos_meta::*;
use leptos_router::*;

use crate::data::{Post, Slug, Draft};
use crate::pages::{
  AboutPage, NotFoundPage, BlogPostPage, BlogPostsPage
};
use crate::components::{Footer, Header};

#[component]
pub fn App() -> impl IntoView {
  // Provides context that manages stylesheets, titles, meta tags, etc.
  provide_meta_context();

  view! {
    // injects a stylesheet into the document <head>
    // id=leptos means cargo-leptos will hot-reload this stylesheet
    <Stylesheet id="leptos" href="/pkg/leptos_start.css"/>

    // sets the document title
    <Title text="Einargs' Blog"/>

    // content for this welcome page
    <Router>
      <div class="min-h-screen flex flex-col">
        <Header />
        <div class="grow flex flex-col bg-paper xl:container">
          <main class="p-5 grow flex flex-col">
            <BlogRouter />
          </main>
          <Footer />
        </div>
      </div>
    </Router>
  }
}

#[component(transparent)]
fn BlogRouter() -> impl IntoView {
  view! {
    <Routes>
      <StaticRoute
        path="/"
        view=AboutPage
        static_params=|| Box::pin(async { StaticParamsMap::default() })
      />
      <StaticRoute
        path="/blog"
        view=BlogPostsPage
        static_params=|| Box::pin(async { StaticParamsMap::default() })
      />
      <StaticRoute
        path="/blog/:slug"
        view=BlogPostPage
        static_params=|| Box::pin(async { run_system(blog_static_params) })
        />
      <StaticRoute
        path="/404"
        view=NotFoundPage
        static_params=|| Box::pin(async { StaticParamsMap::default() })
        />
    </Routes>
  }
}

/// Counter.
/// To do anything dynamic we need an island.
#[island]
fn Counter() -> impl IntoView {
  // Creates a reactive value to update the button
  let (count, set_count) = create_signal(0);
  let on_click = move |_| set_count.update(|count| *count += 1);
  view! {
    <button on:click=on_click>"Click Me: " {count}</button>
  }
}

fn blog_static_params(
  post_slugs: Query<&Slug, (With<Post>, Without<Draft>)>
) -> StaticParamsMap {
  let mut map = StaticParamsMap::default();
  map.insert(
    "slug".to_string(),
    post_slugs.iter().map(|slug| slug.0.clone()).collect(),
  );
  map
}
